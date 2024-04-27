
export function load_merkle_graph_js(passed_fake_data){

    console.log(passed_fake_data)

    let fake_data = {
        hash: 'hash 1-1',
        hash_label: "hash 1-1",
        children: [
            {
                hash: 'hash 1-1',
                hash_label: "hash 1-1",
                children: [
                    {
                        hash: 'hash 1-1',
                        hash_label: "hash 1-1",
                        children: []
                    },
                    {
                        hash: 'hash 1-1',
                        hash_label: "hash 1-1",
                        children: []
                    }
                ]
            },
            {
                hash: 'hash 1-1',
                hash_label: "hash 1-1",
                children: []
            }
        ]
    }

    let svg_holder = document.getElementById("merkle_graph_holder")
    let width = svg_holder.clientWidth;
    svg_holder.style.height = `${width / 2}px`; // make height same as width
    let height = svg_holder.clientHeight;

    console.log(svg_holder)

    let margin = {top: 50, right: 0, bottom: 50, left: 0}
    let innerWidth = width - margin.left - margin.right
    let innerHeight = height - margin.top - margin.bottom

    let svg = d3.select('#merkle_graph_holder svg')
        .attr('width', width)
        .attr('height', height)

    let zoom_g = svg
        .append('g')
    
    let svg_g = zoom_g.append('g')
        .attr('transform', `translate(${margin.left},${margin.top})`)

    let tree = d3.tree().size([innerWidth, innerHeight])
    let root = d3.hierarchy(passed_fake_data)
    let links = tree(root).links()

    svg.call(d3.zoom().on('zoom', (e) => {
        zoom_g.attr('transform', e.transform)
    }))

    svg_g.selectAll('path')
        .data(links)
        .enter()
        .append('path')
        .attr('d', d => {
            let halfway_y = (innerHeight - (d.target.y + d.source.y) / 2)
            let line1 = 'M' + d.source.x + ',' + (innerHeight - d.source.y) + ',' + d.source.x  + ' ' + halfway_y;
            let line2 = 'M' + d.source.x  + ' ' + halfway_y + ' ' + d.target.x  + ' ' + halfway_y;
            let line3 = 'M' + d.target.x + ',' + halfway_y + ',' + d.target.x  + ' ' + (innerHeight - d.target.y);
            return line1 + line2 + line3
        })

    svg_g.selectAll('rect')
        .data(root.descendants())
        .join("rect")
            .attr('x', d => d.x)
            .attr('y', d => innerHeight - d.y)
            .attr("rx", 2)
            .attr("ry", 2)
            .attr("width", 150)
            .attr("height", 60)
            .attr("transform", "translate(-75, -30)")

    svg_g.selectAll('text')
        .data(root.descendants())
        .join('text')
            .attr('x', d => d.x)
            .attr('y', d => innerHeight - d.y)
            .text(d => d.data.text)
            .attr('text-anchor', 'middle')
            .attr('dominant-baseline', "middle")
            .attr('dy', '-13px')

    // svg_g.selectAll('hash')
    //     .data(root.descendants())
    //     .join('text')
    //         .attr('x', d => d.x)
    //         .attr('y', d => innerHeight - d.y)
    //         .text(d => d.data.hash_label)
    //         .attr('text-anchor', 'middle')
    //         .attr('dominant-baseline', "middle")
    //         .attr('dy', '+13px')
        
}