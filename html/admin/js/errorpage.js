export function loadErrorPage() {
    document
        .getElementById("content")
        .innerHTML = getErrorPageContent();

}
function getErrorPageContent() {
    return `
    <div class="error-message text">
        <center><h1>Error</h1>
      ⚠️ Something went wrong. Please try again.
      </center>
    </div>`;
}
