const url = new URL("http://localhost:3000/api/auth/authorize");
url.searchParams.append("client_id", "plupp");
url.searchParams.append("redirect_uri", "http://localhost:3000/api/goodbye");
url.searchParams.append("response_type", "code");

console.log(await (await fetch(url)).text());
