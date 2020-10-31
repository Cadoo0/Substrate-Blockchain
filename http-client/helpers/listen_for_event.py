import websocket
import json
try:
    import thread
except ImportError:
    import _thread as thread
import time


def on_message(ws, message):
    print(message)


def on_error(ws, error):
    print(error)


def on_close(ws):
    print("### closed ###")


def on_open(ws):
    def run(*args):
        # ws.send(json.dumps({
        #     'id': 1,
        #     'jsonrpc': '2.0',
        #     'method': 'chain_getBlockHash',
        #     'params': [
        #         0
        #     ]
        # }))
        # ws.send(json.dumps({
        #     'id': 1,
        #     'jsonrpc': '2.0',
        #     'method': 'state_getRuntimeVersion',
        #     'params': []
        # }))
        # ws.send(json.dumps({
        #     'id': 1,
        #     'jsonrpc': '2.0',
        #     'method': 'system_chain',
        #     'params': []
        # }))
        # ws.send(json.dumps({
        #     'id': 1,
        #     'jsonrpc': '2.0',
        #     'method': 'system_properties',
        #     'params': []
        # }))
        # ws.send(json.dumps({
        #     'id': 1,
        #     'jsonrpc': '2.0',
        #     'method': 'state_subscribeRuntimeVersion',
        #     'params': []
        # }))
        ws.send(json.dumps({
            'id': 1,
            'jsonrpc': '2.0',
            'method': 'rpc_methods',
            'params': []
        }))
    thread.start_new_thread(run, ())


if __name__ == "__main__":
    websocket.enableTrace(True)
    ws = websocket.WebSocketApp("ws://127.0.0.1:9944", on_message=on_message, on_error=on_error, on_close=on_close, on_open=on_open)
    ws.run_forever()