const shutdownBtn = document.getElementById('shutdownBtn');
const restartBtn = document.getElementById('restartBtn');
const statusDiv = document.getElementById('status');

function showStatus(message, type) {
    statusDiv.textContent = message;
    statusDiv.className = `status ${type}`;

    setTimeout(() => {
        statusDiv.textContent = '';
        statusDiv.className = 'status';
    }, 5000);
}

function disableButtons() {
    shutdownBtn.disabled = true;
    restartBtn.disabled = true;
}

function enableButtons() {
    shutdownBtn.disabled = false;
    restartBtn.disabled = false;
}

async function executeCommand(endpoint, action) {
    const confirmed = confirm(`Are you sure you want to ${action} the PC?`);

    if (!confirmed) {
        return;
    }

    disableButtons();
    showStatus(`Executing ${action}...`, 'info');

    try {
        const response = await fetch(endpoint, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
        });

        const data = await response.json();

        if (data.success) {
            showStatus(`${action} command sent successfully!`, 'success');
        } else {
            showStatus(`Error: ${data.message}`, 'error');
            enableButtons();
        }
    } catch (error) {
        showStatus(`Network error: ${error.message}`, 'error');
        enableButtons();
    }
}

shutdownBtn.addEventListener('click', () => {
    executeCommand('/api/shutdown', 'shutdown');
});

restartBtn.addEventListener('click', () => {
    executeCommand('/api/restart', 'restart');
});
