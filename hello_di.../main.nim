import asynchttpserver, asyncdispatch, os
import std/strformat

var i: int = 1

proc sendFile(req: Request, filepath: string, contentType: string) {.async.} =
    if fileExists(filepath):
        let content = readFile(filepath)
        let headers = newHttpHeaders([("Content-Type", contentType)])
        await req.respond(Http200, content, headers)
    else:
        await req.respond(Http404, "Файл " & filepath & " не найден")


proc handleRequest(req: Request) {.async.} =
    case req.url.path
    of "/", "/index.html":
        await req.sendFile("index.html", "text/html; charset=utf-8")

    of "/style.css":
        await req.sendFile("style.css", "text/css; charset=utf-8")

    of "/hello_dick":
        await req.respond(Http200, &"Привет писька. В {i} раз", newHttpHeaders([("Content-Type", "text/text; charset=utf-8")]))
        i += 1

    of "/update_dick":
        await req.respond(Http200, &"Привет писька. В {i} раз", newHttpHeaders([("Content-Type", "text/text; charset=utf-8")]))

    else:
        await req.respond(Http404, "Страница не найдена")

proc main() {.async.} =
  var server = newAsyncHttpServer()
  await server.serve(Port(8080), handleRequest, address = "0.0.0.0")

waitFor main()