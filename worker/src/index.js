export default {
  async fetch(request, env) {
    try {
      const { pathname } = new URL(request.url);
      let segments = pathname.split("/");
      console.log(segments);
      if (segments[1] == "releases" && segments.length == 4) {
          let gh_request = releases(segments[2], segments[3])
          let gh_response = fetch(gh_request);
          return gh_response;
       } else if (segments[1] == "downloads" && segments.length == 6) {
          let gh_request = downloads(segments[2], segments[3], segments[4], segments[5])
          let gh_response = fetch(gh_request);
          return gh_response;
       } else {
          return new Response(index(request), { status: 400 });
       }
    } catch(e) {
      return new Response(e.stack, { status: 500 });
    }
  }
}

function index(request) {
    return `
    >o_o< >o_o< >o_o< >o_o< >o_o< >o_o< >o_o< >o_o< >o_o< >o_o< >o_o< >o_o< >o_o< >o_o<


    welcome to octolotl v0.0.0
    a proxy for github, by axodotdev


    ROUTES
    ======

    releases                GET /releases/{owner}/{repo_name}
    release downloads       GET /dowloads/{owner}/{repo_name}/{tag}/{filename}


    >o_o< >o_o< >o_o< >o_o< >o_o< >o_o< >o_o< >o_o< >o_o< >o_o< >o_o< >o_o< >o_o< >o_o<




    * received an unhandlable request:

    ${request.method} ${request.url}`
}

function releases(owner, repo) {
    let github_url = "https://api.github.com/repos/" + owner + "/" + repo + "/releases";
    let headers = new Headers({
        "User-Agent" : "axodotdev-octolotl"
    });
    return new Request(github_url, { headers: headers });
}

function downloads(owner, repo, tag, filename) {
    let github_url = "https://github.com/" + owner + "/" + repo + "/releases/download/" + tag + "/" + filename;
    let headers = new Headers({
        "User-Agent" : "axodotdev-octolotl"
    });
    return new Request(github_url, { headers: headers });
}
