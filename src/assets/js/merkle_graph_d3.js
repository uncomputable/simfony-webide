
export function load_merkle_graph_js(tree_data){

    let horizontal = true;
    let nodeSize = [162, 40] // x, y
    let nodeGap = [10, 40]
    if (horizontal) 
        nodeGap = [nodeGap[1], nodeGap[0]];

    let svg_holder = document.getElementById("merkle_graph_holder")
    svg_holder.innerHTML = "";

    let nodeCount = countNodes(tree_data)
    if (nodeCount > 1200){
        let div = document.createElement("div");
        div.innerText = `Too many nodes to display graph. Node count: ${nodeCount}`;
        div.classList.add("graph-error");
        svg_holder.appendChild(div);
        return
    }

    let svg_el = document.createElementNS("http://www.w3.org/2000/svg", "svg");
    svg_holder.appendChild(svg_el);

    let merkleContainer = document.getElementById("merkle-container");
    let width = merkleContainer.clientWidth;
    let height = width / 2
    svg_holder.style.height = `${height}px`;

    let svg = d3.select('#merkle_graph_holder svg')
        .attr('width', width)
        .attr('height', height)

    let zoom_g = svg.append('g')

    let centerGraph = horizontal ? `translate(${nodeSize[0]}, ${height / 2})` : `translate(${width / 2}, ${nodeSize[1]})`;
    let svg_g = zoom_g.append('g')
        .attr('transform',centerGraph);

    let nodePositions = horizontal ? [nodeSize[1] + nodeGap[1], nodeSize[0] + nodeGap[0]] : [nodeSize[0] + nodeGap[0], nodeSize[1] + nodeGap[1]]
    let tree = d3.tree()
        .nodeSize(nodePositions)
    let root = d3.hierarchy(tree_data)
    let links = tree(root).links()

    svg.call(d3.zoom().on('zoom', (e) => {
        zoom_g.attr('transform', e.transform)
    }))    

    svg_g.selectAll('path')
        .data(links)
        .enter()
        .append('path')
            .attr('d', d => {
                let halfway_y = (d.target.y + d.source.y) / 2
                return horizontal
                    ? `M${d.source.y} ${d.source.x} ${halfway_y} ${d.source.x} ${halfway_y} ${d.target.x} ${d.target.y} ${d.target.x}`
                    : `M${d.source.x} ${d.source.y} ${d.source.x} ${halfway_y} ${d.target.x} ${halfway_y}  ${d.target.x} ${d.target.y}`
            })

    let node_groups = svg_g.selectAll('g')
        .data(root.descendants())
        .join("g")
        
    node_groups.append("rect")
        .attr('x', d => horizontal ? d.y : d.x)
        .attr('y', d => horizontal ? d.x : d.y)
        .attr('rx', 5)
        .attr('ry', 5)
        .attr('width', nodeSize[0])
        .attr('height', nodeSize[1])
        .attr('transform', `translate(-${nodeSize[0]/2}, -${nodeSize[1]/2})`)
        .attr('class', 'node-rect')

    node_groups.append('text')
        .attr('x', d => horizontal ? d.y : d.x)
        .attr('y', d => horizontal ? d.x : d.y)
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
        .attr('x', d => horizontal ? d.y : d.x)
        .attr('y', d => horizontal ? d.x : d.y)
        .attr('text-anchor', 'middle')
        .attr('dominant-baseline', 'middle')
        .attr('class', 'node-full-text')
        .text(d => d.data.text)
}

function countNodes(node, count = 0){
    count++
    for (let n of node.children){
        count += countNodes(n)
    }
    return count
}