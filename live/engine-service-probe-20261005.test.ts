import assert from 'node:assert/strict';
import {test} from 'node:test';
import {readFileSync} from 'node:fs';
test('engine-owned lifecycle probe marker',()=>assert.equal(readFileSync(new URL('./engine-service-probe-20261005.txt',import.meta.url),'utf8'),"Harmless engine-owned lifecycle/restart probe: engine-owned-pr-gate-20261005-v1\n"));
