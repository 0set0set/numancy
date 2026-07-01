const NOTE_SECTIONS = {
  feat: "Features",
  fix: "Bug Fixes",
  perf: "Performance Improvements",
  refactor: "Code Refactoring",
};

function parseSquashCommitLine(line) {
  const match = line.match(/^\s*(?:[-*]\s+)?(?:[a-f0-9]{7,40}\s+)?([a-z]+)(?:\(([^)]+)\))?(!)?:\s(.+)$/i);
  if (!match) {
    return null;
  }

  const [, rawType, scope, breaking, subject] = match;
  const type = rawType.toLowerCase();
  if (!NOTE_SECTIONS[type]) {
    return null;
  }

  return {
    type,
    scope,
    breaking: Boolean(breaking),
    subject: subject.trim(),
  };
}

function collectNotes(commits) {
  const notesByType = new Map();

  for (const commit of commits) {
    for (const line of commit.message.split("\n")) {
      const note = parseSquashCommitLine(line);
      if (!note) {
        continue;
      }

      const notes = notesByType.get(note.type) ?? [];
      notes.push(note);
      notesByType.set(note.type, notes);
    }
  }

  return notesByType;
}

function formatNote(note) {
  const scope = note.scope ? `**${note.scope}:** ` : "";
  const breaking = note.breaking ? "**BREAKING:** " : "";
  return `* ${scope}${breaking}${note.subject}`;
}

module.exports = {
  generateNotes: async (pluginConfig, context) => {
    const notesByType = collectNotes(context.commits);
    const sections = [];

    for (const [type, title] of Object.entries(NOTE_SECTIONS)) {
      const notes = notesByType.get(type);
      if (!notes?.length) {
        continue;
      }

      sections.push(`### ${title}\n\n${notes.map(formatNote).join("\n")}`);
    }

    return sections.join("\n\n");
  },
};
