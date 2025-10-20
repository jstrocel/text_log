const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

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

// Load and display current directory
async function loadCurrentDirectory() {
  try {
    const directory = await invoke('get_directory');
    document.getElementById('current-directory').textContent = `Save Location: ${directory}`;
  } catch (error) {
    document.getElementById('current-directory').textContent = 'Save Location: Error loading directory';
  }
}

// Change directory handler
async function changeDirectory() {
  try {
    await invoke('choose_directory');
    // Directory update will be handled by the event listener
  } catch (error) {
    alert(`Error opening directory picker: ${error}`);
  }
}

// Listen for directory changes from the backend
listen('directory-changed', (event) => {
  document.getElementById('current-directory').textContent = `Save Location: ${event.payload}`;
});

// Initialize directory display on page load
loadCurrentDirectory();

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

document.getElementById('change-location-btn').addEventListener('click', changeDirectory);