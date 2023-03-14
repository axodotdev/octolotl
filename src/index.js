export default {
  async fetch(request, env) {
    const GH_API_URL = "https://api.github.com";
    const GH_URL = "https://github.com";
    try {
      const { pathname } = new URL(request.url);
      let segments = pathname.split("/");
      if (segments[1] == "releases") {
        let endpoint = segments[2];
        let headers = new Headers({
          "User-Agent": "axodotdev-cf-worker-releases",
        });
        let github_url;

        if (endpoint === "download") {
          let [_, __, ___, org, repo, version, file] = segments;
          github_url = `${GH_URL}/${org}/${repo}/releases/download/${version}/${file}`;
        } else {
          let [_, __, org, repo] = segments;
          github_url = `${GH_API_URL}/repos/${org}/${repo}/releases`;
        }

        let gh_request = new Request(github_url, { headers: headers });

        let gh_response = fetch(gh_request);
        return gh_response;
      } else {
        return new Response("Unsupported", { status: 400 });
      }
    } catch (e) {
      return new Response(e.stack, { status: 500 });
    }
  },
};
