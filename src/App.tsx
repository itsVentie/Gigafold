import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface FileEntry {
  name: string;
  size: number;
  is_dir: boolean;
}

export default function App() {
  const [files, setFiles] = useState<FileEntry[]>([]);

  useEffect(() => {
    async function loadFiles() {
      try {
        const fetchedFiles: FileEntry[] = await invoke('get_files', { path: '/' });
        setFiles(fetchedFiles);
      } catch (error) {
        console.error(error);
      }
    }
    loadFiles();
  }, []);

  return (
    <div style={{ padding: '20px', fontFamily: 'sans-serif', background: '#1e1e1e', color: '#fff', minHeight: '100vh' }}>
      <h2>Gigafold Storage</h2>
      <ul style={{ listStyleType: 'none', padding: 0 }}>
        {files.map((file) => (
          <li key={file.name} style={{ padding: '8px 0', borderBottom: '1px solid #333' }}>
            <span>{file.is_dir ? '📁 ' : '📄 '}</span>
            <strong>{file.name}</strong>
            {!file.is_dir && <span style={{ color: '#aaa' }}> ({file.size} bytes)</span>}
          </li>
        ))}
      </ul>
    </div>
  );
}
