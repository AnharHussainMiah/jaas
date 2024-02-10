

pub fn index_js() -> &'static str {
r#"
import run from './service.js'

const stdin = new TextDecoder().decode(await Deno.readAll(Deno.stdin));
if(Deno.args.length > 0){
    const sessionId = Deno.args[0];
    const servieName = Deno.args[1]
    const JaaS = {};
    JaaS.Http = function(x) {
        // todo: make http calls
        console.log(`==> [${servieName.toUpperCase()}][HTTP] ${x}`);
    };
    JaaS.Db = function(x) {
        // todo: make db API call
        console.log(`==> [${servieName.toUpperCase()}][SQL] ${x}`);
    };
    JaaS.Log = function(x) {
        console.log(`==> [${servieName.toUpperCase()}][LOG] ${x}`)
    }
    const data = JSON.parse(stdin);
    const result = run(JaaS, data);
    Deno.writeTextFileSync(`data/${servieName}/${sessionId}`, JSON.stringify(result));
}
"#
}