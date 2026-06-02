import init, { HittableList, Camera } from './pkg/rt_wasm.js';

// ── Scene state ───────────────────────────────────────────────────────────
const spheres = [
  { mat: 'matte', x: 0.0, y: -100.5, z: -1.0, r: 100.0, r_col: 0.8, g_col: 0.8, b_col: 0.0, ground: true },
  { mat: 'metal', x: 0.0, y:    0.0, z: -1.5, r:   0.5, r_col: 0.8, g_col: 0.8, b_col: 0.8, fuzz: 0.0,   ground: false },
];

let activeMat = 'matte';
let wasmReady = false;

// ── DOM refs ──────────────────────────────────────────────────────────────
const aspectSelect  = document.getElementById('aspect-select');
const widthInput    = document.getElementById('width-input');
const heightDisplay = document.getElementById('height-display');
const sphereListEl  = document.getElementById('sphere-list');
const sphereCount   = document.getElementById('sphere-count');
const renderBtn     = document.getElementById('render-btn');
const canvas        = document.getElementById('img');
const placeholder   = document.getElementById('placeholder');
const statusDot     = document.getElementById('status-dot');
const canvasInfo    = document.getElementById('canvas-info');
const progressFill  = document.getElementById('progress-fill');
const dimW          = document.getElementById('dim-w');
const dimH          = document.getElementById('dim-h');

// ── Helpers ───────────────────────────────────────────────────────────────
const getAspect = () => parseFloat(aspectSelect.value);
const getWidth  = () => parseInt(widthInput.value) || 800;
const getHeight = () => Math.max(1, Math.round(getWidth() / getAspect()));
const fv        = id  => parseFloat(document.getElementById(id).value) || 0;

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

function getCamParams() {
  return {
    samples: parseInt(document.getElementById('cam-samples').value) || 250,
    depth:   parseInt(document.getElementById('cam-depth').value)   || 50,
    vfov:    parseFloat(document.getElementById('cam-vfov').value)  || 30.0,
    from: [ fv('cam-from-x'), fv('cam-from-y'), fv('cam-from-z') ],
    at:   [ fv('cam-at-x'),   fv('cam-at-y'),   fv('cam-at-z')   ],
    up:   [ fv('cam-up-x'),   fv('cam-up-y') || 1, fv('cam-up-z') ],
  };
}

// ── Panel collapse ────────────────────────────────────────────────────────
document.querySelectorAll('.panel-header').forEach(header => {
  header.addEventListener('click', () => {
    const body = document.getElementById(header.dataset.panel);
    if (!body) return;
    const closing = header.classList.contains('open');
    header.classList.toggle('open', !closing);
    body.classList.toggle('collapsed', closing);
  });
});

// ── Material tabs ─────────────────────────────────────────────────────────
document.querySelectorAll('.mat-tab').forEach(tab => {
  tab.addEventListener('click', () => {
    document.querySelectorAll('.mat-tab').forEach(t => t.classList.remove('active'));
    document.querySelectorAll('.mat-fields').forEach(f => f.classList.remove('active'));
    tab.classList.add('active');
    activeMat = tab.dataset.mat;
    document.getElementById(`fields-${activeMat}`).classList.add('active');
  });
});

// ── Sphere list ───────────────────────────────────────────────────────────
function matLabel(s) {
  if (s.mat === 'matte')  return `matte · rgb(${fmt(s.r_col)}, ${fmt(s.g_col)}, ${fmt(s.b_col)})`;
  if (s.mat === 'metal')  return `metal · rgb(${fmt(s.r_col)}, ${fmt(s.g_col)}, ${fmt(s.b_col)}) · fuzz=${fmt(s.fuzz)}`;
  if (s.mat === 'glass')  return `glass · ior=${fmt(s.idr)}`;
  if (s.mat === 'hollow') return `hollow · t=${fmt(s.thickness)}`;
  return s.mat;
}

function renderSphereList() {
  sphereCount.textContent = spheres.length;
  if (spheres.length === 0) {
    sphereListEl.innerHTML = '<div class="empty-state">no spheres in scene</div>';
    return;
  }
  sphereListEl.innerHTML = spheres.map((s, i) => `
    <div class="sphere-item">
      <div class="sphere-dot" data-mat="${s.ground ? 'ground' : s.mat}"></div>
      <div class="sphere-info">
        <div class="sphere-coords">(${fmt(s.x)}, ${fmt(s.y)}, ${fmt(s.z)}) &nbsp;r=${fmt(s.r)}</div>
        <div class="sphere-meta">${matLabel(s)}</div>
      </div>
      <button class="sphere-remove" data-i="${i}" title="Remove">✕</button>
    </div>
  `).join('');

  sphereListEl.querySelectorAll('.sphere-remove').forEach(btn => {
    btn.addEventListener('click', () => {
      spheres.splice(parseInt(btn.dataset.i), 1);
      renderSphereList();
    });
  });
}

// ── Add sphere ────────────────────────────────────────────────────────────
document.getElementById('add-btn').addEventListener('click', () => {
  const x = parseFloat(document.getElementById('sx').value) || 0;
  const y = parseFloat(document.getElementById('sy').value) || 0;
  const z = parseFloat(document.getElementById('sz').value) || -1;
  const r = Math.abs(parseFloat(document.getElementById('sr').value) || 0.5);

  const s = { mat: activeMat, x, y, z, r, ground: false };

  if (activeMat === 'matte') {
    s.r_col = parseFloat(document.getElementById('matte-r').value) || 0;
    s.g_col = parseFloat(document.getElementById('matte-g').value) || 0;
    s.b_col = parseFloat(document.getElementById('matte-b').value) || 0;
  } else if (activeMat === 'metal') {
    s.r_col = parseFloat(document.getElementById('metal-r').value) || 0;
    s.g_col = parseFloat(document.getElementById('metal-g').value) || 0;
    s.b_col = parseFloat(document.getElementById('metal-b').value) || 0;
    s.fuzz  = parseFloat(document.getElementById('metal-fuzz').value) || 0;
  } else if (activeMat === 'glass') {
    s.idr = parseFloat(document.getElementById('glass-idr').value) || 1.5;
  } else if (activeMat === 'hollow') {
    s.thickness = parseFloat(document.getElementById('hollow-thickness').value) || 0.1;
  }

  spheres.push(s);
  renderSphereList();
});

// ── Watchers ──────────────────────────────────────────────────────────────
aspectSelect.addEventListener('change', updateDims);
widthInput.addEventListener('input', updateDims);

// ── Status / progress ─────────────────────────────────────────────────────
function setStatus(state, msg) {
  statusDot.className = 'status-dot' + (state ? ' ' + state : '');
  canvasInfo.textContent = msg;
}
function setProgress(pct, indeterminate = false) {
  progressFill.className = 'progress-fill' + (indeterminate ? ' indeterminate' : '');
  if (!indeterminate) progressFill.style.width = pct + '%';
}

// ── Render ────────────────────────────────────────────────────────────────
renderBtn.addEventListener('click', async () => {
  if (!wasmReady) {
    setStatus('rendering', 'Initialising WASM…');
    setProgress(0, true);
    renderBtn.disabled = true;
    try {
      await init();
      wasmReady = true;
    } catch (e) {
      setStatus('', 'Failed to load WASM module');
      setProgress(0);
      renderBtn.disabled = false;
      return;
    }
  }

  renderBtn.disabled = true;
  setStatus('rendering', 'Rendering…');
  setProgress(0, true);
  await new Promise(r => setTimeout(r, 10));

  try {
    const aspect_ratio = getAspect();
    const width  = getWidth();
    const height = getHeight();
    const cam    = getCamParams();

    canvas.width  = width;
    canvas.height = height;

    const ctx   = canvas.getContext('2d');
    const world = HittableList.new();

    spheres.forEach(s => {
      if (s.mat === 'matte')  world.add_matte_sphere(s.x, s.y, s.z, s.r, s.r_col, s.g_col, s.b_col);
      if (s.mat === 'metal')  world.add_metal_sphere(s.x, s.y, s.z, s.r, s.r_col, s.g_col, s.b_col, s.fuzz);
      if (s.mat === 'glass')  world.add_glass_sphere(s.x, s.y, s.z, s.r, s.idr);
      if (s.mat === 'hollow') world.add_hollow_glass_sphere(s.x, s.y, s.z, s.r, s.thickness);
    });

    const camObj = Camera.new();
    camObj.aspect_ratio      = aspect_ratio;
    camObj.image_width       = width;
    camObj.samples_per_pixel = cam.samples;
    camObj.max_depth         = cam.depth;
    camObj.vfov              = cam.vfov;
    camObj.set_lookfrom(cam.from[0], cam.from[1], cam.from[2]);
    camObj.set_lookat(cam.at[0], cam.at[1], cam.at[2]);
    camObj.set_vup(cam.up[0], cam.up[1], cam.up[2]);

    const img = camObj.render(world);

    setProgress(60);
    ctx.beginPath();
    for (let row = 0; row < height; row++) {
      for (let col = 0; col < width; col++) {
        ctx.fillStyle = img[row * width + col];
        ctx.fillRect(col, row, 1, 1);
      }
    }
    ctx.stroke();

    setProgress(100);
    placeholder.style.display = 'none';
    canvas.style.display = 'block';
    setStatus('ready', `Rendered ${width}×${height}`);
  } catch (e) {
    setStatus('', 'Render error — check console');
    console.error(e);
  }

  renderBtn.disabled = false;
  setTimeout(() => setProgress(0), 800);
});

// ── Init ──────────────────────────────────────────────────────────────────
updateDims();
renderSphereList();
setStatus('', 'Ready to render');
