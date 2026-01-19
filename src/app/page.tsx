"use client";

import { useEffect, useState, useRef } from "react";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

export default function Home() {
  const [label, setLabel] = useState<string | null>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    // Detect window label
    const window = getCurrentWebviewWindow();

    // Defer to avoid synchronous state update during effect body
    setTimeout(() => {
      setLabel(window.label);
    }, 0);

    // Auto-focus logic for HUD
    if (window.label === "hud") {
      const interval = setInterval(() => {
        if (inputRef.current) {
          inputRef.current.focus();
          clearInterval(interval);
        }
      }, 50);
      return () => clearInterval(interval);
    }
  }, []);

  if (label === "hud") {
    return (
      <div className="hud-container">
        <div className="hud-search-wrapper">
          <span className="hud-sigil">🐷</span>
          <input
            ref={inputRef}
            className="hud-input"
            placeholder="Search notes or type a command..."
            spellCheck={false}
          />
          <kbd className="hud-shortcut">Ctrl K</kbd>
        </div>
        <div className="hud-results-stub">
          <div className="result-item selected">
            <span className="icon">📄</span>
            <span className="title">Welcome to Pigpen</span>
            <span className="metadata">Recently Opened</span>
          </div>
          <div className="result-item">
            <span className="icon">⚙️</span>
            <span className="title">System Preferences</span>
            <span className="metadata">Command</span>
          </div>
        </div>
        <style jsx>{`
          .hud-container {
            height: 100vh;
            background: var(--bg-hud);
            border: 1px solid var(--border-color);
            border-radius: var(--win11-radius);
            display: flex;
            flex-direction: column;
            box-shadow: var(--win11-shadow);
          }
          .hud-search-wrapper {
            display: flex;
            align-items: center;
            padding: 18px 24px;
            border-bottom: 1px solid var(--border-color);
          }
          .hud-sigil {
            font-size: 24px;
            margin-right: 16px;
          }
          .hud-input {
            flex: 1;
            background: transparent;
            border: none;
            color: var(--text-primary);
            font-size: 20px;
            outline: none;
            font-family: inherit;
          }
          .hud-shortcut {
            background: rgba(255, 255, 255, 0.05);
            padding: 4px 8px;
            border-radius: 4px;
            font-size: 12px;
            color: var(--text-secondary);
          }
          .hud-results-stub {
            padding: 8px;
            flex: 1;
            overflow-y: auto;
          }
          .result-item {
            display: flex;
            align-items: center;
            padding: 12px 16px;
            border-radius: 8px;
            cursor: pointer;
            margin-bottom: 4px;
          }
          .result-item.selected {
            background: rgba(255, 255, 255, 0.1);
          }
          .result-item .icon {
            margin-right: 16px;
            font-size: 18px;
          }
          .result-item .title {
            flex: 1;
            font-size: 14px;
            font-weight: 500;
          }
          .result-item .metadata {
            font-size: 12px;
            color: var(--text-secondary);
          }
        `}</style>
      </div>
    );
  }

  return (
    <div className="dashboard-container">
      <nav className="dashboard-sidebar">
        <div className="sidebar-header">Pigpen</div>
        <div className="sidebar-links">
          <div className="link active">🏠 Inbox</div>
          <div className="link">📚 Vault</div>
          <div className="link">⚙️ Settings</div>
        </div>
      </nav>
      <main className="dashboard-content">
        <h1>Dashboard</h1>
        <p>The persistent organizational layer for your knowledge vault.</p>
      </main>
      <style jsx>{`
        .dashboard-container {
          display: flex;
          height: 100vh;
          background: var(--bg-dashboard);
        }
        .dashboard-sidebar {
          width: 240px;
          border-right: 1px solid var(--border-color);
          padding: 24px;
        }
        .sidebar-header {
          font-size: 20px;
          font-weight: 700;
          margin-bottom: 32px;
          color: var(--accent-color);
        }
        .sidebar-links .link {
          padding: 10px 14px;
          border-radius: 8px;
          margin-bottom: 4px;
          cursor: pointer;
          font-size: 14px;
        }
        .sidebar-links .link.active {
          background: rgba(255, 77, 77, 0.1);
          color: var(--accent-color);
        }
        .dashboard-content {
          flex: 1;
          padding: 48px;
        }
        h1 { margin-bottom: 16px; }
      `}</style>
    </div>
  );
}
