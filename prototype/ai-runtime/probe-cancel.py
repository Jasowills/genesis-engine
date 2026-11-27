import time, threading
import numpy as np, onnx
from onnx import helper, TensorProto
import onnxruntime as ort

N = 20000
inp = helper.make_tensor_value_info('x', TensorProto.FLOAT, [4])
out = helper.make_tensor_value_info('y', TensorProto.FLOAT, [4])
init = helper.make_tensor('one', TensorProto.FLOAT, [4], [1.0]*4)
nodes = [helper.make_node('Add', ['x' if i == 0 else f't{i-1}', 'one'], [f't{i}' if i < N-1 else 'y']) for i in range(N)]
g = helper.make_graph(nodes, 'loop', [inp], [out], [init])
m = helper.make_model(g, opset_imports=[helper.make_opsetid('', 17)], ir_version=8)
onnx.checker.check_model(m)
onnx.save(m, 'probe-cancel.onnx')
print('model nodes', N, 'size', __import__('os').path.getsize('probe-cancel.onnx'))
so = ort.SessionOptions()
so.graph_optimization_level = ort.GraphOptimizationLevel.ORT_DISABLE_ALL
s = ort.InferenceSession('probe-cancel.onnx', so, providers=['CPUExecutionProvider'])
x = np.zeros(4, dtype=np.float32)

t0=time.perf_counter(); r=s.run(None, {'x':x}); print('full run sec',round(time.perf_counter()-t0,3), 'out',r[0])

for trial in range(3):
    ro = ort.RunOptions()
    result = {}
    def work():
        try:
            t0=time.perf_counter()
            result['out']=s.run(None, {'x':x}, ro)
            result['elapsed']=time.perf_counter()-t0
        except Exception as e:
            result['exc']=f'{type(e).__name__}: {str(e)[:200]}'
            result['elapsed']=time.perf_counter()-t0
    th=threading.Thread(target=work); th.start()
    time.sleep(0.002)
    t1=time.perf_counter(); ro.terminate=True; print('terminate set after',round(t1-t0 if 't0' in dir() else 0,3))
    th.join(timeout=30)
    print('trial',trial, result.get('exc','NO EXCEPTION'), 'elapsed',round(result.get('elapsed',0),3))
