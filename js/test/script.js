window.addEventListener("load", () => {
    const canvas = document.getElementById("canvas");
    const ctx    = canvas.getContext("2d", { alpha: false });
    
    ctx.imageSmoothingEnabled = false;

    const txtFrames = document.getElementById("frames");
    const txtSize   = document.getElementById("size");
    const txtLen    = document.getElementById("length");

    const upload    = document.getElementById("file");

    const btnPlay   = document.getElementById("btn-play");
    const btnStop   = document.getElementById("btn-stop");

    const chkLoop   = document.getElementById("chk-loop");
    const chkInvert = document.getElementById("chk-invert");

    const reader    = new FileReader();

    let video;

    function loadBuff(buffer) {
        video = minvideo.Video.fromData(buffer);

        txtSize.innerText   = `${video.width}x${video.height}`;
        txtFrames.innerText = video.getFrameAmount();
        txtLen.innerText    = buffer.length;
    }

    let playbackId = 0;

    function play() {
        if(!video) return;
        
        const id   = playbackId + 1;
        playbackId = id;

        for(let frameI = 0; frameI < video.getFrameAmount(); frameI++) {
            setTimeout(() => {
                if(playbackId !== id) return;

                const frame = video.getFrame(frameI);
                
                txtFrames.innerText = frameI + "/" + video.getFrameAmount();

                for(let y = 0; y < frame.height; y++) {
                    for(let x = 0; x < frame.width; x++) {
                        const c = frame.getColor(x, y);
    
                        if(chkInvert.checked) ctx.fillStyle = `rgb(${c.b},${c.g},${c.r})`;
                        else                  ctx.fillStyle = `rgb(${c.r},${c.g},${c.b})`;

                        const w = canvas.width  / video.width;
                        const h = canvas.height / video.height;

                        const rX = x * w;
                        const rY = y * h;

                        ctx.fillRect(rX, rY, w, h);
                    }
                } 
            }, ( (1 / 30) * frameI) * 1000 );
        }

        setTimeout(() => {
            if(chkLoop.checked) play();
        }, video.getFrameAmount() * (1/ 30) * 1000);
    }

    function stop() {
        playbackId++;
    }

    reader.onload = function() {
        const buff = new Uint8Array(this.result);
        loadBuff(buff);
    }

    file.addEventListener("change", () => {
        if(upload.files.length === 0) return;
        const file = upload.files[0];

        const buff = reader.readAsArrayBuffer(file); // passed to reader.onload
    });

    btnPlay.addEventListener("click", play);
    btnStop.addEventListener("click", stop);

});