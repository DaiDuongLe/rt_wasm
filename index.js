import init, { HittableList, Camera } from './pkg/rt_wasm.js';

const spheres = [
  { x: 0.0, y: 0.0,    z: -1.0, r: 0.5,   ground: false },
  { x: 0.0, y: -100.5, z: -1.0, r: 100.0, ground: true  }
];

const aspectSelect  = document.getElementById('aspect-select');
const widthInput    = document.getElementById('width-input');
const heightDisplay = document.getElementById('height-display');
const sphereList    = document.getElementById('sphere-list');
const sphereCount   = document.getElementById('sphere-count');
const renderBtn     = document.getElementById('render-btn');
const canvas        = document.getElementById('img');
const placeholder   = document.getElementById('placeholder');
const statusDot     = document.getElementById('status-dot');
const canvasInfo    = document.getElementById('canvas-info');
const progressFill  = document.getElementById('progress-fill');
const dimW          = document.getElementById('dim-w');
const dimH          = document.getElementById('dim-h');

let wasmReady = false;

function getAspect() { return parseFloat(aspectSelect.value); }
function getWidth()  { return parseInt(widthInput.value) || 800; }
function getHeight() {
  const h = Math.round(getWidth() / getAspect());
  return h < 1 ? 1 : h;
}

function fmt(v) {
  const n = parseFloat(v);
  return (n % 1 === 0) ? n.toFixed(1) : parseFloat(n.toFixed(3)).toString();
}

function updateDims() {
  const h = getHeight();
  heightDisplay.textContent = h;
  dimW.textContent = getWidth();
  dimH.textContent = h;
}

function renderSphereList() {
  sphereCount.textContent = spheres.length;
  if (spheres.length === 0) {
    sphereList.innerHTML = '<div class="empty-state">no spheres in scene</div>';
    return;
  }
  sphereList.innerHTML = spheres.map((s, i) => `
    <div class="sphere-item">
      <div class="sphere-dot ${s.ground ? 'ground' : ''}"></div>
      <div class="sphere-info">
        <div class="sphere-coords">(${fmt(s.x)}, ${fmt(s.y)}, ${fmt(s.z)})</div>
        <div class="sphere-radius">r = ${fmt(s.r)}</div>
      </div>
      <button class="sphere-remove" data-i="${i}" title="Remove">✕</button>
    </div>
  `).join('');

  sphereList.querySelectorAll('.sphere-remove').forEach(btn => {
    btn.addEventListener('click', () => {
      spheres.splice(parseInt(btn.dataset.i), 1);
      renderSphereList();
    });
  });
}

document.getElementById('add-btn').addEventListener('click', () => {
  const x = parseFloat(document.getElementById('sx').value) || 0;
  const y = parseFloat(document.getElementById('sy').value) || 0;
  const z = parseFloat(document.getElementById('sz').value) || -1;
  const r = Math.abs(parseFloat(document.getElementById('sr').value) || 0.5);
  spheres.push({ x, y, z, r, ground: false });
  renderSphereList();
});

aspectSelect.addEventListener('change', updateDims);
widthInput.addEventListener('input', updateDims);

function setStatus(state, msg) {
  statusDot.className = 'status-dot ' + state;
  canvasInfo.textContent = msg;
}

function setProgress(pct, indeterminate = false) {
  progressFill.className = 'progress-fill' + (indeterminate ? ' indeterminate' : '');
  if (!indeterminate) progressFill.style.width = pct + '%';
}

renderBtn.addEventListener('click', async () => {
  if (!wasmReady) {
    setStatus('rendering', 'Initialising WASM…');
    setProgress(0, true);
    renderBtn.disabled = true;
    try {
      await init();
      wasmReady = true;
    } catch(e) {
      setStatus('', 'Failed to load WASM module');
      setProgress(0);
      renderBtn.disabled = false;
      return;
    }
  }

  renderBtn.disabled = true;
  setStatus('rendering', 'Rendering…');
  setProgress(0, true);

  await new Promise(r => setTimeout(r, 10)); // let paint flush

  try {
    const aspect_ratio = getAspect();
    const width = getWidth();
    let height = Math.round(width / aspect_ratio);
    height = height < 1 ? 1 : height;

    canvas.width  = width;
    canvas.height = height;

    const ctx = canvas.getContext('2d');
    const world = HittableList.new();
    spheres.forEach(s => world.add_sphere(s.x, s.y, s.z, s.r));

    const cam = Camera.new();
    cam.aspect_ratio  = aspect_ratio;
    cam.image_width   = width;
    const img = cam.render(world);

    setProgress(60);
    const SIZE = 1;
    ctx.beginPath();
    for (let row = 0; row < height; row++) {
      for (let col = 0; col < width; col++) {
        const idx = row * width + col;
        ctx.fillStyle = img[idx];
        ctx.fillRect(col, row, SIZE, SIZE);
      }
    }
    ctx.stroke();

    setProgress(100);
    placeholder.style.display = 'none';
    canvas.style.display = 'block';
    setStatus('ready', `Rendered in ${width}×${height}`);
  } catch(e) {
    setStatus('', 'Render error — check console');
    console.error(e);
  }

  renderBtn.disabled = false;
  setTimeout(() => setProgress(0), 800);
});

// Init
updateDims();
renderSphereList();
setStatus('', 'Load WASM and hit Render');
