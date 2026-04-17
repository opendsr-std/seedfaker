#!/usr/bin/env bash
set -euo pipefail
echo "--- npm package ---"
node -e "
  const {SeedFaker}=require('./packages/npm');
  const f=new SeedFaker({seed:'test-pkg',until:2025});
  const v=f.field('name');
  if(!v)throw new Error('field returned empty');
  const n=SeedFaker.fields().length;
  if(n<200)throw new Error('fields: '+n);
  const fp=SeedFaker.fingerprint();
  if(!fp.startsWith('sf0-'))throw new Error('fingerprint: '+fp);
  const r=f.records(['name','email'],{n:3,ctx:'strict'});
  if(r.length!==3)throw new Error('records: '+r.length);
  const rec=f.record(['name','email'],{ctx:'strict'});
  if(!rec.name||!rec.email)throw new Error('record: '+JSON.stringify(rec));
  f.validate(['name','email','phone:e164']);
  try{f.validate(['name:e164']);throw new Error('should reject')}catch(e){if(e.message==='should reject')throw e;}
  const u=f.field('name',{upper:true});
  if(u!==u.toUpperCase())throw new Error('modifier upper: '+u);
  const xr=f.records(['name:upper'],{n:1});
  if(xr[0]['name_upper']!==xr[0]['name_upper'].toUpperCase())throw new Error('records transform');
  console.log('  ok: name='+v+', fields='+n+', fp='+fp);
"
