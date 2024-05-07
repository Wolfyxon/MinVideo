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

    function play() {
        if(!video) return;
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

});