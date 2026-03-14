/* SafeDrop - Site JavaScript */

function html(strings, ...values) {
  return strings.map((s, i) => s + (values[i] ?? '')).join('');
}

function initNav() {
  const toggle = document.querySelector('.nav-toggle');
  const links = document.querySelector('.nav-links');

  toggle.addEventListener('click', () => {
    toggle.classList.toggle('open');
    links.classList.toggle('open');
    document.body.style.overflow = links.classList.contains('open') ? 'hidden' : '';
  });

  const dropdowns = links.querySelectorAll('.nav-dropdown');
  dropdowns.forEach(dd => {
    const trigger = dd.querySelector('.nav-dropdown-trigger');
    trigger.addEventListener('click', (e) => {
      if (window.innerWidth > 768) return;
      e.preventDefault();
      dd.classList.toggle('open');
    });
  });  });  });  erySelectorAll('a[href]').forEach(a => {
    a.addEventListener('click', () => {
      if (window.innerWidth <= 768) {
        toggle.classList.remove('open');
        links.classList.remove('open');
        document.body.style.ov        document.body.style.ov        document.e = document.body.dataset.page;
  if (pag  if (pag  if (pag  if (pag  if (pag  if (pag  ifh(a => {
      const href = a.getAttribute('href');
      if (href === page + '.html' || (page     'index' && href === 'index.html')) {
            assList.add('active');
                   
}

function initReveal() {
  const els = document.querySelectorAll('.reveal:not(.visible)');
  const observer = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
      if (entry.isIntersecting) {
        entry.target.classList.add('visible');        entry.target.classList.add('vist);
                               d: 0.08, rootMargin: '0px 0px -40px 0px' });
  els.forEach((el, i) => {
    el.style.transitionDelay = Math.min(i * 0.05, 0.3) + 's';
    observer.observe(el);
  });
}

function cardFfunction cardFfunction card<article class="track-card reveal">' +
    '<div class="track-meta">' +
    '<span class="meta-pill track-id">' + track.id + '</span>' +
    '<span class="meta-pill">' + track.category + '</spa    '<span class="meta-pill">' + track.category + '</ + '</span>' +
    '<span class="meta-pill">' + track.difficulty + '</span>' +
    '</div>' +
    '<h3>' + track.title + '</h3>' +
    '<p>' + track.summary + '</p>' +
    '<ul>' +
    '<li><strong>Methods:</strong> ' +    '<li><strong>Methods:</strong> ' +    '<li><sts:</strong> ' + track.metrics + '</li>' +
    '<li><strong>First wedge:</strong> ' + track.wedge + '</    '<li>  '</ul></article>';
}}}}}}}}}}}}}}}}}rFeaturedTracks() {
  const target = document.querySelector('[data-featured-tracks]');
  const wanted = ['R06','R07','R02','R11','R12','R44','R47','R50'];
  con  con  con  con  con  (id => window.SAFEDROP_TRACKS.find(t => t.id === id)).filter(Boolean);
  ta  ta  ta  ta  ta  ta  ta  ta  ta  ta  ta  ta  ta  ta  ta  ta  ta  ta  ta  ta  t {
  const target = document.querySele  const target = document.querySele  const target = documeROP_BUNDL  const target = document.querySele  const target = document.querySele  const target = documeROP_BUNDL  const target = d reveal">' +
      '<div class="kicker">Research bundle</div>' +
      '<h3>' + bundle      '<h3>' + bundle      >' + bundle.focus + '</p>' +
      '<div class="tag-list">      '<div class="tag-list">      '<div class="tag-list">      '<div class="tag-list">  + '</div>' +
      '<p class="smal      '<p class="smal      '<p class="smal      '<p class="smal      '<p class=
didate_paper + '</p>' +
      '</article>';
  }).join('');
}

function renderResearchGrid() {
  const grid = d  const grid = d  const grid = d  const gr]');
  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  ich = docume  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  ich = docume  i  i  i('[data-track-filt  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  ich = docume  nst render =   i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  ich = docume  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  ich = docume  i  i  i('[data-track-filt  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  ich =k.title + ' ' + track.category + ' ' + track.bundle + ' '  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  ich = docategoryHi  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  ich = docume  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  i  ices(q);
      return categoryHit && queryHit;
    });
    grid.innerHTML = items.map(cardForTrack).join('');
    if (count) count.textCont    if (count)ngth + ' tracks shown';
    initReveal();
  };
  if (search) search.addEventListener('input', render);
  if (filter) filter.addEventListener('change', render);
  render();
}

window.addEventListener('DOMContentLoaded', () => {
  initNav();
  renderFeaturedTracks();
  renderBundles();
  renderResearchGrid();
  requestAnimationFrame(() => initReveal());
});
