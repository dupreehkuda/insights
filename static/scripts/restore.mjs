import { restoreLink } from "./restore-link.mjs";

const form = document.getElementById("restoreForm");
const input = document.getElementById("oldLink");
const error = document.getElementById("linkError");
const result = document.getElementById("result");
const output = document.getElementById("newLink");
const open = document.getElementById("openLink");
const status = document.getElementById("copyStatus");

document.getElementById("restoreButton").disabled = false;

input.addEventListener("input", () => {
    result.hidden = true;
    error.hidden = true;
    input.removeAttribute("aria-invalid");
    status.textContent = "";
});

form.addEventListener("submit", (event) => {
    event.preventDefault();
    const link = restoreLink(input.value, window.location.origin);
    result.hidden = true;
    error.hidden = true;
    input.removeAttribute("aria-invalid");
    status.textContent = "";

    if (!link) {
        error.textContent = "This doesn't look like an Insights event link. Copy the full event or summary link from Telegram and try again.";
        error.hidden = false;
        input.setAttribute("aria-invalid", "true");
        input.focus();
        return;
    }

    output.value = link;
    open.href = link;
    result.hidden = false;
    open.focus();
});

document.getElementById("copyLink").addEventListener("click", async () => {
    try {
        await navigator.clipboard.writeText(output.value);
        status.textContent = "Link copied.";
    } catch {
        output.focus();
        output.select();
        status.textContent = "Select and copy the new link above.";
    }
});
