const { invoke } = window.__TAURI__.core;

const quill = new Quill('#editor-container', {
  theme: 'snow',
  placeholder: 'Start writing...',
  modules: {
    toolbar: [
      ['bold', 'italic', 'underline'],
      ['link'],
      [{ 'list': 'ordered'}, { 'list': 'bullet' }],
      ['clean']
    ]
  }
});

document.getElementById('send-btn').addEventListener('click', async () => {
  const content = quill.getText(); // Plain text
  // Or use quill.root.innerHTML for HTML
  
  try {
    const filename = await invoke('save_entry', { content });
    alert(`Saved to ${filename}`);
    quill.setText(''); // Clear editor
  } catch (error) {
    alert(`Error: ${error}`);
  }
});