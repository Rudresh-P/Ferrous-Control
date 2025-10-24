import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { QRCodeSVG } from "qrcode.react";
import "./App.css";
import AutoStartToggle from "./components/AutoStartToggle";

function App() {
  const [status, setStatus] = useState({ message: "", type: "" });
  const [buttonsDisabled, setButtonsDisabled] = useState(false);
  const [modal, setModal] = useState({ show: false, title: "", message: "", onConfirm: null });
  const [localIp, setLocalIp] = useState("");

  useEffect(() => {
    async function fetchLocalIp() {
      try {
        const ip = await invoke("get_local_ip");
        setLocalIp(ip);
      } catch (error) {
        console.error("Failed to get local IP:", error);
        setLocalIp("Unable to get IP");
      }
    }
    fetchLocalIp();
  }, []);

  function showModal(title, message) {
    return new Promise((resolve) => {
      setModal({ show: true, title, message, onConfirm: resolve });
    });
  }

  function closeModal(result) {
    if (modal.onConfirm) {
      modal.onConfirm(result);
    }
    setModal({ show: false, title: "", message: "", onConfirm: null });
  }

  async function handleShutdown() {
    const confirmed = await showModal("Confirm Action", "Are you sure you want to shutdown the PC?");

    if (!confirmed) {
      return;
    }

    setButtonsDisabled(true);
    setStatus({ message: "Executing shutdown...", type: "info" });

    try {
      const result = await invoke("shutdown");
      if (result.success) {
        setStatus({ message: "Shutdown command sent successfully!", type: "success" });
      } else {
        setStatus({ message: `Error: ${result.message}`, type: "error" });
        setButtonsDisabled(false);
      }
    } catch (error) {
      setStatus({ message: `Error: ${error}`, type: "error" });
      setButtonsDisabled(false);
    }

    setTimeout(() => setStatus({ message: "", type: "" }), 5000);
  }

  async function handleSleep() {
    const confirmed = await showModal("Confirm Action", "Are you sure you want to put the PC to sleep?");

    if (!confirmed) {
      return;
    }

    setButtonsDisabled(true);
    setStatus({ message: "Executing sleep...", type: "info" });

    try {
      const result = await invoke("sleep");
      if (result.success) {
        setStatus({ message: "Sleep command sent successfully!", type: "success" });
      } else {
        setStatus({ message: `Error: ${result.message}`, type: "error" });
        setButtonsDisabled(false);
      }
    } catch (error) {
      setStatus({ message: `Error: ${error}`, type: "error" });
      setButtonsDisabled(false);
    }

    setTimeout(() => setStatus({ message: "", type: "" }), 5000);
  }

  async function handleCancel() {
    setStatus({ message: "Cancelling shutdown...", type: "info" });

    try {
      const result = await invoke("cancel_shutdown");
      if (result.success) {
        setStatus({ message: result.message, type: "success" });
      } else {
        setStatus({ message: `Error: ${result.message}`, type: "error" });
      }
    } catch (error) {
      setStatus({ message: `Error: ${error}`, type: "error" });
    }

    setTimeout(() => setStatus({ message: "", type: "" }), 5000);
  }

  return (
    <main className="container">
      <AutoStartToggle />
      <h1>Ferrous Control</h1>
      <p className="subtitle">Remote PC Control Panel</p>
      {localIp && (
        <div className="network-info">
          <p className="ip-address">Network Address: http://{localIp}:7777</p>
          <div className="qr-code-container">
            <QRCodeSVG
              value={`http://${localIp}:7777`}
              size={180}
              level="H"
            />
            <p className="qr-label">Scan to connect from mobile device</p>
          </div>
        </div>
      )}

      <div className="button-container">
        <button
          className="control-btn shutdown-btn"
          onClick={handleShutdown}
          disabled={buttonsDisabled}
        >
          <span className="icon">🔴</span>
          <span>Shutdown</span>
        </button>

        <button
          className="control-btn sleep-btn"
          onClick={handleSleep}
          disabled={buttonsDisabled}
        >
          <span className="icon">😴</span>
          <span>Sleep</span>
        </button>

        <button
          className="control-btn cancel-btn"
          onClick={handleCancel}
        >
          <span className="icon">⛔</span>
          <span>Cancel</span>
        </button>
      </div>

      {status.message && (
        <div className={`status ${status.type}`}>
          {status.message}
        </div>
      )}

      {modal.show && (
        <div className="modal-overlay active" onClick={(e) => {
          if (e.target.className.includes('modal-overlay')) {
            closeModal(false);
          }
        }}>
          <div className="modal">
            <h2>{modal.title}</h2>
            <p>{modal.message}</p>
            <div className="modal-buttons">
              <button className="modal-btn modal-btn-cancel" onClick={() => closeModal(false)}>
                Cancel
              </button>
              <button className="modal-btn modal-btn-confirm" onClick={() => closeModal(true)}>
                Confirm
              </button>
            </div>
          </div>
        </div>
      )}
    </main>
  );
}

export default App;
