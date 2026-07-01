const RELEASE_BY_TYPE = {
  feat: "minor",
  fix: "patch",
  perf: "patch",
  refactor: "patch",
  chore: "patch",
};

const RELEASE_PRIORITY = {
  patch: 1,
  minor: 2,
  major: 3,
};

function higherRelease(current, next) {
  if (!next) {
    return current;
  }

  if (!current || RELEASE_PRIORITY[next] > RELEASE_PRIORITY[current]) {
    return next;
  }

  return current;
}

function releaseForLine(line) {
  const match = line.match(/^\s*(?:[-*]\s+)?(?:[a-f0-9]{7,40}\s+)?([a-z]+)(?:\([^)]+\))?(!)?:\s.+$/i);
  if (!match) {
    return null;
  }

  const [, type, breaking] = match;
  if (breaking) {
    return "major";
  }

  return RELEASE_BY_TYPE[type.toLowerCase()] ?? null;
}

module.exports = {
  analyzeCommits: async (pluginConfig, context) => {
    let release = null;

    for (const commit of context.commits) {
      if (/BREAKING[- ]CHANGE:/i.test(commit.message)) {
        release = higherRelease(release, "major");
        continue;
      }

      for (const line of commit.message.split("\n")) {
        release = higherRelease(release, releaseForLine(line));
      }
    }

    if (release) {
      context.logger.log("Detected %s release from squash commit details", release);
    }

    return release;
  },
};
