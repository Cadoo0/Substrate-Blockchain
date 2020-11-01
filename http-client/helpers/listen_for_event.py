import websocket
import json
try:
    import thread
except ImportError:
    import _thread as thread
import time


def on_message(ws, message):
    print(json.dumps(json.loads(message), indent=4))


def on_error(ws, error):
    print(error)


def on_close(ws):
    print('Connection closed')


def on_open(ws):
    def run(*args):
        ws.send(json.dumps({
            'id': 1,
            'jsonrpc': '2.0',
            'method': 'state_subscribeStorage',
            'params': [
                ['0x26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7']
            ]
        }))
    thread.start_new_thread(run, ())


if __name__ == "__main__":
    websocket.enableTrace(True)
    ws = websocket.WebSocketApp("ws://127.0.0.1:9944", on_message=on_message, on_error=on_error, on_close=on_close, on_open=on_open)
    ws.run_forever()