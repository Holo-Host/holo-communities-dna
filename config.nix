let
 release-commit = "7628d3827d4cf03611404c16ba039f0853b5fed1";
 current = "0.0.39-alpha4";
 previous = "0.0.39-alpha3";
 # tag will ultimately be current version when it hits holonix
 # https://github.com/holochain/holonix/blob/master/release/default.nix#L7
 tag = "v${current}";
 holonix-version = "v0.0.44";
 holonix-sha256 = "0819439idwhdbavmlcy99c2ai5d9a0k7rbimbsk47p9vndw3s6cy";
in
rec {

 # configure holonix itself
 holonix = {

  # true = use a github repository as the holonix base (recommended)
  # false = use a local copy of holonix (useful for debugging)
  use-github = true;

  # configure the remote holonix github when use-github = true
  github = {

   # can be any github ref
   # branch, tag, commit, etc.
   ref = holonix-version;

   # the sha of what is downloaded from the above ref
   # note: even if you change the above ref it will not be redownloaded until
   #       the sha here changes (the sha is the cache key for downloads)
   # note: to get a new sha, get nix to try and download a bad sha
   #       it will complain and tell you the right sha
   sha256 = holonix-sha256;

   # the github owner of the holonix repo
   owner = "holochain";

   # the name of the holonix repo
   repo = "holonix";
  };

  # configuration for when use-github = false
  local = {
   # the path to the local holonix copy
   path = ../holonix;
  };

 };

 release = {
  hook = {
   # sanity checks before deploying
   # to stop the release
   # exit 1
   preflight = ''
'';

   # bump versions in the repo
   version = ''
cargo update
'';

   # publish artifacts to the world
   publish = ''
echo "go look at circle for binary building and crates publishing!"
'';
  };

  # the commit hash that the release process should target
  # this will always be behind what ends up being deployed
  # the release process needs to add some commits for changelog etc.
  commit = release-commit;

  # the semver for prev and current releases
  # the previous version will be scanned/bumped by release scripts
  # the current version is what the release scripts bump *to*
  version = {
   current = current;
   # not used by version hooks in this repo
   previous = previous;
  };

  github = {
   # markdown to inject into github releases
   # there is some basic string substitution {{ xxx }}
   # - {{ changelog }} will inject the changelog as at the target commit
   template = ''
'';

   # owner of the github repository that release are deployed to
   owner = "holochain";

   # repository name on github that release are deployed to
   repo = "hylo-holo-dnas";

   # canonical local upstream name as per `git remote -v`
   upstream = "origin";

  };

  # non-standard, overridden by holonix internally anyway
  # used by check artifacts
  tag = tag;
 };
}