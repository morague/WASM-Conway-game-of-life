var width;
var height;

window.addEventListener('resize', () => {
  window.location.reload()
});

function get_grid_dimension(block, pixels) {
  let w = block.offsetWidth;
  let h = block.offsetHeight;
  console.log(w, h)

  width = Math.round(w/pixels)
  height =Math.round(h/pixels)

  return width, height    
}


