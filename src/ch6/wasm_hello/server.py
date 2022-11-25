import http.server, socketserver
# MIME에 application/wasm을 추가
Handler = http.server.SimpleHTTPRequestHandler
Handler.extensions_map['.wasm'] = 'application/wasm'
# 서버 시작
port = 8888
with socketserver.TCPServer(("", port), Handler) as d:
    print("[Running] http://localhost:{}".format(port))
    try:
        d.serve_forever()
    except:
        pass
    finally:
        d.server_close()
