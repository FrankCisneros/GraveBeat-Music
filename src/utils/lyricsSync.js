export function parseLyrics(lrcText) {
    if (!lrcText) return [];

    const lines = lrcText.split(/\r?\n/);
    const parsed = [];

    // Pattern to match [mm:ss.xx]
    const timeRegExp = /\[(\d{2}):(\d{2})\.(\d{2,3})\]/;

    for (const line of lines) {
        const match = line.match(timeRegExp);
        if (match) {
            const minutes = parseInt(match[1], 10);
            const seconds = parseInt(match[2], 10);
            const milliseconds = match[3].length === 2 ? parseInt(match[3], 10) * 10 : parseInt(match[3], 10);

            const timeInSeconds = minutes * 60 + seconds + milliseconds / 1000;
            const text = line.replace(timeRegExp, '').trim();

            parsed.push({
                time: timeInSeconds,
                text: text
            });
        }
    }

    // Some lines might not have timestamps (plain lyrics mixed with LRC)
    // We sort them by time just in case, ignoring those without time
    return parsed.sort((a, b) => a.time - b.time);
}

export function getActiveLyricIndex(parsedLyrics, currentTime) {
    if (!parsedLyrics || parsedLyrics.length === 0) return -1;

    for (let i = parsedLyrics.length - 1; i >= 0; i--) {
        if (currentTime >= parsedLyrics[i].time) {
            return i;
        }
    }

    return -1;
}
