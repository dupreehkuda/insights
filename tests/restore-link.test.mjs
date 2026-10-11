import test from "node:test";
import assert from "node:assert/strict";
import { restoreLink } from "../static/scripts/restore-link.mjs";

const origin = "https://events.example";
const id = "12345678-1234-4321-9876-123456789abc";

test("restores event and summary links without changing their identifiers", () => {
    for (const kind of ["insights", "summary"]) {
        const path = `/${kind}/${id}`;
        for (const prefix of ["http://217.25.93.207", "https://217.25.93.207", "217.25.93.207", "http://217.25.93.207:8080", "", origin]) {
            assert.equal(restoreLink(`  ${prefix}${path}  `, origin), origin + path);
        }
        assert.equal(restoreLink(`http://217.25.93.207${path}/?next=https://other.example#fragment`, origin), origin + path);
    }
});

test("rejects other destinations, non-event routes, malformed IDs and injected markup", () => {
    for (const input of [
        "", "not a link", "http://217.25.93.207", `http://other.example/insights/${id}`,
        `//other.example/insights/${id}`, `http://217.25.93.207.evil.example/insights/${id}`,
        `http://217.25.93.207@other.example/insights/${id}`, `http://user:pass@217.25.93.207/insights/${id}`,
        `ftp://217.25.93.207/insights/${id}`, `javascript:alert(1)`, `data:text/html,<script>alert(1)</script>`,
        `http://217.25.93.207:9000/insights/${id}`, `/api/v1/event/${id}`, `/insights/not-a-uuid`,
        `/INSIGHTS/${id}`, `/insights/${id}/extra`, `/insights/${id}%22`, `/insights/${id}\n/other`,
        `http://217.25.93.207\\@other.example/insights/${id}`, "a".repeat(2049),
    ]) assert.equal(restoreLink(input, origin), null, input);
});
