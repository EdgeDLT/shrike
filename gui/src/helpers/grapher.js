import cytoscape from 'cytoscape';
import { Formatter } from './formatter';
import cytoscapeDagre from 'cytoscape-dagre';
import cytoscapePopper from 'cytoscape-popper';

cytoscape.use(cytoscapeDagre);
cytoscape.use(cytoscapePopper)

export class Grapher {
    static getNodesFromAddresses(addresses) {
        return addresses.map(a => {
            return { data: { id: a } }
        })
    }

    static getEdgesFromTransfers(transfers) {
        return transfers.map(t => {
            let i = t.from.slice(0,5) + t.to.slice(0,5)
            return { data: { id: i, weight: t.amount, source: t.from, target: t.to } }
        })
    }

    static default(id){

      return cytoscape({
        container: document.getElementById(id),

        boxSelectionEnabled: false,
        autounselectify: true,

        style: cytoscape.stylesheet()
          .selector('node')
            .style({
              'content': 'data(id)',
              'background-color': "#61bffc",
              'color': "white"
            })
          .selector('edge')
            .style({
              'curve-style': 'segments',
              'target-arrow-shape': 'triangle',
              'width': 2,
              'line-color': '#ddd',
              'line-opacity': 0.5,
              'target-arrow-color': '#ddd'
            })
          .selector('.highlighted')
            .style({
              'background-color': '#61bffc',
              'line-color': '#61bffc',
              'target-arrow-color': '#61bffc',
              'transition-property': 'background-color, line-color, target-arrow-color',
              'transition-duration': '0.5s'
            }),

        elements: {
            nodes: [
              { data: { id: "Hello" } },
              { data: { id: "World" } }
            ],
            edges: [
              { data: { id: "HelloWorld", weight: 1, source: "Hello", target: "World" } }
            ]
          },

        layout: {
          name: 'dagre',
          fit: true,
          padding: 20,
          nodeDimensionsIncludeLabels: true,
        }
      })
    }

    static draw(nodes, edges) {

      let primary = Formatter.parseIfHsl(getComputedStyle(document.documentElement).getPropertyValue('--primary'))
      let secondary = Formatter.parseIfHsl(getComputedStyle(document.documentElement).getPropertyValue('--secondary'))

      return cytoscape({
        container: document.getElementById("cyto"),

        boxSelectionEnabled: false,
        autounselectify: true,

        style: cytoscape.stylesheet()
          .selector('node')
            .style({
              'content': 'data(id)',
              'background-color': primary,
              'color': primary
            })
          .selector('edge')
            .style({
              'curve-style': 'bezier',
              'target-arrow-shape': 'triangle',
              'width': 2,
              'line-color': '#ddd',
              'line-opacity': 0.5,
              'target-arrow-color': secondary
            })
          .selector('.highlighted')
            .style({
              'background-color': '#61bffc',
              'line-color': '#61bffc',
              'target-arrow-color': '#61bffc',
              'transition-property': 'background-color, line-color, target-arrow-color',
              'transition-duration': '0.5s'
            }),

        elements: {
            nodes: nodes,
            edges: edges
          },

        layout: {
          name: 'dagre',
          fit: true,
          padding: 20,
          nodeDimensionsIncludeLabels: true,
        }
      })
    }
}
