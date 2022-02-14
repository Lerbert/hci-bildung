export function upload(callback: (url: string, mimetype: string) => void) {
    const input = document.getElementById('file-input') as HTMLInputElement;

    input.value = '';

    input.onchange = () => {
        if (!input.value || !input.files) return;

        const file = new Blob([input.files[0]], {type: input.files[0].type});
        const url = URL.createObjectURL(file);

        callback(url, file.type);
        
        input.onchange = null;
    };

    input.click();
}