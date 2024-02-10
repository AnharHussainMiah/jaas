🎷 JaaS
===

> JaaS is an open source "JavaScript as a Service" sort of serverless i.e "function as a service" application

## What's this?

`JaaS` allows you to create tiny fully self contained function _(written in JavaScript)_ that can then be deployed "at the push of a button". You don't have to worry about wrangling and configuring servers, just deploy your JavaScript function and have it be ready to use immediately. Of course just an isolated function would be rather limited, so `JaaS` allows each function access to a `sqlite` database as well making full HTTP `GET` and `POST` requests.

Since each service/function gets its own isolated database and access to external HTTP calls, you could in theory create entire miniature applications inside of a `JaaS` function. 


## What features does this have?

* single static binary no other dependency
* dual purpose binary acts as `cli` as well as the `server`
* hassle free installation just drop the `JaaS` binary on server
* each function has its own named path
* each function get its own `sqlite` database access
* each function gets access to http `POST` and http `GET`
* each function can also access request URL and request headers

## Server install

First download the binary:

```shell
$ wget https://github.com/AnharHussainMiah/jaas/releases/jaas
```

now we need to make the binary an executable:

```shell
$ sudo chmod +x jaas
```

and finally we boot up the server:

```shell
$ ./jaas --server
```

This will boot up the `JaaS` in server mode and it will automatically create a `key` as well as a `data/` folder. If you want to backup or migrate to another server all you need to do is just copy the key and data folder that's it!


## How do I create my first `JaaS` service?

Lets say we want to create a "ping-pong" service, lets create a new service:

```shell
$ jaas new ping-pong
$ ==> 🔨 creating new service "ping-pong" ..
$ ==> 🎉 all done! created new ping-pong/ service project
```

This will create a new folder called "ping-pong", just "cd" into this directory, and edit your function inside `src/index.js` example:

```shell
$ cd ping-pong/
$ code src/index.js # use whatever editor you want here I'm using Visual Studio Code
```

Now lets edit the `index.js` file:

```javascript
import { jaas } from '../lib/jaas'

jaas.run(data => {
    return {"reply": "pong"}
});
```
we just need to edit our `jaas.config.json` file to point to our server:

```json
{
    "prod": {
        "server": "https://wwww.my-jaas-server-domain.com",
        "key": "7ef2e111-de60-493c-8b8d-36fe7c631399"
    },
    "local": {
        "server": "http://localhost",
        "key": "4866da56-d579-446a-8cd5-a714ab9606cf"
    }
}
```
Warning: **MAKE SURE YOU NEVER EXPOSE YOUR KEY!!**

Now we can deploy our project:

```shell
$ jaas deploy prod
$ ==> 🔨 deploying your service "ping-pong" to server https://my-jaas-server.com/
$ ==> 🚀 success! we have lift off, service is now live at https://my-jaas-server.com/ping-pong
```