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

// Load and display current file contents
async function loadCurrentFileContents() {
  try {
    const contents = await invoke('read_current_file');
    const today = new Date().toISOString().split('T')[0]; // YYYY-MM-DD format
    
    document.getElementById('file-header').textContent = `Today's Journal (${today}.txt):`;
    
    const fileContentDiv = document.getElementById('file-content');
    const fileContentsDiv = document.getElementById('file-contents');
    
    if (contents.trim() === '') {
      fileContentDiv.textContent = 'No entries yet today. Start writing below!';
      fileContentsDiv.classList.add('empty');
    } else {
      fileContentDiv.textContent = contents;
      fileContentsDiv.classList.remove('empty');
      // Auto-scroll to bottom to show latest entries
      fileContentsDiv.scrollTop = fileContentsDiv.scrollHeight;
    }
  } catch (error) {
    document.getElementById('file-header').textContent = `Today's Journal: Error`;
    document.getElementById('file-content').textContent = `Error loading file: ${error}`;
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
  // Reload file contents when directory changes
  loadCurrentFileContents();
});

// Initialize directory display and file contents on page load
loadCurrentDirectory();
loadCurrentFileContents();

document.getElementById('send-btn').addEventListener('click', async () => {
  const content = quill.getText(); // Plain text
  // Or use quill.root.innerHTML for HTML
  
  try {
    const filename = await invoke('save_entry', { content });
    alert(`Saved to ${filename}`);
    quill.setText(''); // Clear editor
    // Refresh file contents to show new entry
    loadCurrentFileContents();
  } catch (error) {
    alert(`Error: ${error}`);
  }
});

document.getElementById('change-location-btn').addEventListener('click', changeDirectory);