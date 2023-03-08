export default {
  async fetch(request, env) {
    try {
      const { pathname } = new URL(request.url);
      let segments = pathname.split("/");
      let org = segments[1];
      let repo = segments[2];
      let github_url = "https://api.github.com/repos/" + org + "/" + repo + "/releases";
      let headers = new Headers({
        "User-Agent"   : "axodotdev-cf-worker-releases"
      });
      let gh_request = new Request(github_url, { headers: headers });

      let gh_response = fetch(gh_request);
      return gh_response;
    } catch(e) {
      return new Response(e.stack, { status: 500 });
    }
  }
}