import requests
import json

url = "http://localhost:8080/jaas"
# data = """{"a": 100, "b": 69}"""
data = '{"name": "bob smith"}'
r = requests.post(url,data=data)
print(r.text)
