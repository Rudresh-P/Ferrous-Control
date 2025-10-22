import { isEnabled, enable, disable } from "@tauri-apps/plugin-autostart";
import { useEffect, useState } from "react";

export default function AutoStartToggle() {

    const [isAutoStartEnabled, setIsAutoStartEnabled] = useState(false);

    const checkAutoStartStatus = async () => {
        const status = await isEnabled();
        setIsAutoStartEnabled(status);
    }

    const handleToggle = async () => {
        try {
            if (isAutoStartEnabled) {
                await disable();
                const status = await isEnabled();
                setIsAutoStartEnabled(status);
            } else {
                await enable();
                const status = await isEnabled();
                setIsAutoStartEnabled(status);
            }
        } catch (error) {
            console.error("Failed to toggle autostart:", error);
        }
    }

    useEffect(() => {
        checkAutoStartStatus();
    }, []);

    return (
        <div style={{
            display: 'flex',
            alignItems: 'center',
            gap: '12px',
            padding: '16px',
            backgroundColor: '#1e1e1e',
            borderRadius: '8px',
            color: '#fff'
        }}>
            <span style={{ fontSize: '16px', fontWeight: '500' }}>Auto Start</span>

            <label style={{
                position: 'relative',
                display: 'inline-block',
                width: '50px',
                height: '24px',
                cursor: 'pointer'
            }}>
                <input
                    type="checkbox"
                    checked={isAutoStartEnabled}
                    onChange={handleToggle}
                    style={{ opacity: 0, width: 0, height: 0 }}
                />
                <span style={{
                    position: 'absolute',
                    top: 0,
                    left: 0,
                    right: 0,
                    bottom: 0,
                    backgroundColor: isAutoStartEnabled ? '#4CAF50' : '#ccc',
                    borderRadius: '24px',
                    transition: 'background-color 0.3s',
                    cursor: 'pointer'
                }}>
                    <span style={{
                        position: 'absolute',
                        content: '',
                        height: '18px',
                        width: '18px',
                        left: isAutoStartEnabled ? '29px' : '3px',
                        bottom: '3px',
                        backgroundColor: 'white',
                        borderRadius: '50%',
                        transition: 'left 0.3s'
                    }} />
                </span>
            </label>

            <span style={{
                fontSize: '14px',
                color: isAutoStartEnabled ? '#4CAF50' : '#888'
            }}>
                {isAutoStartEnabled ? "Enabled" : "Disabled"}
            </span>
        </div>
    )
}