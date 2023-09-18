import { renderToReadableStream } from "react-dom/server";

Bun.serve({
    port: 3000,
    async fetch(req) {
        const { pathname, searchParams: query } = new URL(req.url)

        if (pathname === "/hello") {
            const name = query.get('name');
            const num1 = Number(query.get('num1'))
            const num2 = Number(query.get('num2'))
    
            if(typeof(name) != 'string' || isNaN(num1) || isNaN(num2))
                return Response.json({
                    error: "invalid type. name:string, num1: number, num2: number"    
                })
    
            return Response.json({
                hello: `Hello ${name}`,
                sum: num1 + num2,
                mul: num1 * num2,
                sub:  num1 - num2,
                div: num2 == 0 ? null : num1 / num2,
            })
        }

        else if (pathname === "/hello_html") {
            const name = query.get('name');
            const num1 = Number(query.get('num1'))
            const num2 = Number(query.get('num2'))
    
            if(typeof(name) != 'string' || isNaN(num1) || isNaN(num2))
                return Response.json({
                    error: "invalid type. name:string, num1: number, num2: number"    
                })

            const stream = await renderToReadableStream(
                <div>
                    <div>
                        <div>hello</div>
                        <div>sum</div>
                        <div>mul</div>
                        <div>sub</div>
                        <div>div</div>
                    </div>
                    <div>
                        <div>{name}</div>
                        <div>{num1 + num2}</div>
                        <div>{num1 * num2}</div>
                        <div>{num1 - num2}</div>
                        <div>{num2 == 0 ? null : num1 / num2}</div>
                    </div>
                </div>
              );
              return new Response(stream, {
                headers: { "Content-Type": "text/html" },
              });

            return 
        }
        

        return new Response("404!");

    },
  });