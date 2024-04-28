
export function load_merkle_graph_js(tree_data){

    let nodeSize = [162, 40] // x, y
    let nodeGap = [40, 10]

    let svg_holder = document.getElementById("merkle_graph_holder")
    let old_svg = document.querySelector('#merkle_graph_holder svg')
    if (old_svg) old_svg.remove()
    let svg_el = document.createElementNS("http://www.w3.org/2000/svg", "svg");
    svg_holder.appendChild(svg_el);

    let width = svg_holder.clientWidth;
    svg_holder.style.height = `${width / 2}px`;
    let height = svg_holder.clientHeight;

    let svg = d3.select('#merkle_graph_holder svg')
        .attr('width', width)
        .attr('height', height)

    let zoom_g = svg.append('g')
    zoom_g.selectAll("*").remove()

    let svg_g = zoom_g.append('g')
        // .attr('transform',`translate(${width / 2}, ${nodeSize[1]})`); // centers graph horizontal layout
        .attr('transform',`translate(${nodeSize[0]}, ${height / 2})`); // centers graph vertical layout

    let tree = d3.tree()
        .nodeSize([nodeSize[1] + nodeGap[1], nodeSize[0] + nodeGap[0]])
    let root = d3.hierarchy(tree_data)
    let links = tree(root).links()

    svg.call(d3.zoom().on('zoom', (e) => {
        zoom_g.attr('transform', e.transform)
    }))    

    svg_g.selectAll('path')
        .data(links)
        .enter()
        .append('path')
            .attr('d', d => { // vertical layout
                let halfway_y = (d.target.y + d.source.y) / 2
                // return `M${d.source.x} ${d.source.y} ${d.source.x} ${halfway_y} ${d.target.x} ${halfway_y}  ${d.target.x} ${d.target.y}`
                return `M${d.source.y} ${d.source.x} ${halfway_y} ${d.source.x} ${halfway_y} ${d.target.x} ${d.target.y} ${d.target.x}`
            })

    let node_groups = svg_g.selectAll('g')
        .data(root.descendants())
        .join("g")
        
    node_groups.append("rect")
        .attr('x', d => d.y)
        .attr('y', d => d.x)
        .attr('rx', 5)
        .attr('ry', 5)
        .attr('width', nodeSize[0])
        .attr('height', nodeSize[1])
        .attr('transform', `translate(-${nodeSize[0]/2}, -${nodeSize[1]/2})`)
        .attr('class', 'node-rect')

    node_groups.append('text')
        .attr('x', d => d.y)
        .attr('y', d => d.x)
        .attr('text-anchor', 'middle')
        .attr('dominant-baseline', 'middle')
        .text(d => {
            if (d.data.text.length > 16)
                return d.data.text.slice(0, 14) + '..'
            else
                return d.data.text
        })
        .attr('class', 'node-main-text')

    // hover elements
    node_groups.append('text')
        .attr('x', d => d.y)
        .attr('y', d => d.x)
        .attr('text-anchor', 'middle')
        .attr('dominant-baseline', 'middle')
        .attr('class', 'node-full-text')
        .text(d => d.data.text)
        
}