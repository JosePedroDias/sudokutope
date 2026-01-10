import { XMLParser, XMLBuilder } from "fast-xml-parser";
import * as fs from "fs";

const FROM = "grid4.svg";
const TO = "grid5.svg";

function simpler(n) {
    simplified += 1;
    return Math.round(Number(n) * 10) / 10;
}

const tagsToProcess = ['path'];
const tagsToVisit = ['g'];
const s = new Set();

function process(nodes) {
    if (!nodes || !Array.isArray(nodes)) return;

    for (const node of nodes) {
        const tag = Object.keys(node).find(key => key !== ':@' && key !== '#text');

        s.add(tag);

        //if (tag !== 'path') continue;

        const attributes = tag && node[':@'];

        if (attributes) {
            const attrKeys = Object.keys(attributes).map(s => s.replace("@@", ""));
            for (const attr of attrKeys) {
                if (attrsToSimplify.includes(attr)) {
                    const escAttr = "@@" + attr;
                    const v = attributes[escAttr];
                    const v2 = simpler(v);
                    attributes[escAttr] = v2;
                    simplified += 1;
                }
            }

            if (attributes['@@d']) {
                attributes['@@d'] = simplifyPath(attributes['@@d']);
            }

            if (attributes['@@opacity']) {
                if (attributes['@@opacity'] === "1") {
                    delete attributes['@@opacity'];
                }   
            }

            if (attributes['@@stroke-width']) {
                if (attributes['@@stroke-width'] === "1") {
                    delete attributes['@@stroke-width'];
                }   
            }

            if (attributes['@@id']) {
                delete attributes['@@id'];
            }

            if (attributes['@@stroke-linejoin']) {
                delete attributes['@@stroke-linejoin'];
            }

            if (attributes['@@style']) {
                let v = attributes['@@style'];
                v = v.replace(/opacity:\s*1/g, "");
                v = v.replace(/stroke-linejoin:\s*round/g, "");
                v = v.replace(/;\s*;/g, ";");
                if (v[0] === ';') v = v.slice(1);
                if (v.length === 0) delete attributes['@@style'];
                else attributes['@@style'] = v;
            }
        }

        if (node[tag] && Array.isArray(node[tag])) {
            process(node[tag]);
        }
    }
}

const options = {
    ignoreAttributes: false,
    preserveOrder: true,
    attributeNamePrefix: "@@", // Prefix to easily identify attributes DEFAULT IS @_
    // :@ key where the attributes get stored
};

const parser = new XMLParser(options);
const builder = new XMLBuilder(options);

const xmlContent = fs.readFileSync(FROM, "utf-8");
const o = parser.parse(xmlContent);

process(o);

console.log(s);

const resultSvg = builder.build(o);
fs.writeFileSync(TO, resultSvg);
