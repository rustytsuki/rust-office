# rust-office
rust-office is an office suite with high compatibility with MS Office (OOXML ECMA-376). The kernel layout engine is written in pure rust. You can open and edit the docx, xlsx, pptx file with rust-office both in desktop and browser driven by wasm.

how to develop:

#### run desktop native app:

`cargo run -p native`

#### run web wasm app:

###### first, start web dev server:

`cd web`

`npm ci` 

`npm run debug`

###### second, start file drive server:

change project root folder,

`cargo run -p server -- --proxy true`

third, open link in a browser:

http://127.0.0.1:4080/

#### Note:

if you want to have hardware acceleration, you need to install opengl driver. On linux, you have to install mesa-dri-drivers. Or rust-office will use software render instead.