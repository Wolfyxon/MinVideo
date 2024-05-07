window.addEventListener("load", () => {
    const canvas = document.getElementById("canvas");
    const ctx = canvas.getContext("2d", { alpha: false });

    const txtFrames = document.getElementById("frames");
    const txtSize = document.getElementById("size");

    const upload = document.getElementById("file");
    const btnPlay = document.getElementById("btn-play");

    const reader = new FileReader();

    let video;

    function loadBuff(buffer) {
        video = minvideo.Video.fromData(buffer);

        txtSize.innerText = `${video.width}x${video.height}`;
        txtFrames.innerText = video.getFrameAmount();
    }

    let playbackId = 0;
    function play() {
        if(!video) return;
        
        const id = playbackId + 1;
        playbackId = id;

        for(let frameI = 0; frameI < video.getFrameAmount(); frameI++) {
            setTimeout(() => {
                if(playbackId !== id) return;

                const frame = video.getFrame(frameI);
                
                for(let y = 0; y < frame.width; y++) {
                    for(let x = 0; x < frame.height; x++) {
                        const c = frame.getColor(x, y);
                        if(!c.r) continue; // TODO: Fix color containing undefined values
    
                        ctx.fillStyle = `rgb(${c.r},${c.g},${c.b})`;
                        ctx.fillRect(x, y, 1, 1);
                    }
                } 
            }, ( (1 / 30) * frameI) * 1000 );
        }
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

});