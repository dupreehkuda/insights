const eventPath = /^\/(insights|summary)\/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}\/?$/;

export function restoreLink(value, origin) {
    const input = value.trim();
    if (!input || input.length > 2048 || /[\s\\]/.test(input)) return null;

    try {
        const current = new URL(origin);
        const source = new URL(
            input.startsWith("/") || /^[a-z][a-z0-9+.-]*:/i.test(input)
                ? input
                : `http://${input}`,
            current.origin,
        );
        const isLegacy = source.hostname === "217.25.93.207"
            && ["", "80", "443", "8080"].includes(source.port);
        const isCurrent = source.origin === current.origin;

        if (!["http:", "https:"].includes(source.protocol)
            || source.username || source.password
            || (!isLegacy && !isCurrent)
            || !eventPath.test(source.pathname)) return null;

        // Only carry the event route across, never a host, query or fragment.
        return new URL(source.pathname.replace(/\/$/, ""), current.origin).href;
    } catch {
        return null;
    }
}
