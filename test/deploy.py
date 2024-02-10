import requests
import json

url = "http://localhost:8080/_/deploy"
key = "c249cbdd-9243-467b-9699-0c191748f157"

payload = """
function run(JaaS, data) {
    JaaS.Log("my spanking new service!");
    return `hello your name is ${data.name}`;
}

export default run;
"""

data = {
    "service": "echo",
    "payload": payload
}
headers = {"x-jaas-key": key}
print("==> deploying service echo...")
r = requests.post(url, data=json.dumps(data), headers=headers)
print(r.text)