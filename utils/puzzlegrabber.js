javascript: (() => {
    if (typeof puzzle == 'object' && puzzle instanceof Puzzle) {
        result = puzzle.serialize();
    } else {
        const pagecontent = document.documentElement.innerHTML;
        const wittle_before = "window.deserializePuzzle(\"";
        const witnesspuzzles_before = "Puzzle.deserialize(\"";
        
        let start_idx = pagecontent.indexOf(wittle_before);
        if (start_idx != -1) {
            const end_idx = pagecontent.indexOf("\")", start_idx);
            result = window.deserializePuzzle(pagecontent.substring(start_idx + wittle_before.length, end_idx)).serialize();
        } else {
            start_idx = pagecontent.indexOf(witnesspuzzles_before);
            if (start_idx == -1) {
                alert("Found no puzzle");
                return;
            }
            const end_idx = pagecontent.indexOf("\")", start_idx);
            result = pagecontent.substring(start_idx + witnesspuzzles_before.length, end_idx);
        }
    }

    navigator.clipboard.writeText(result.replace(/\\/g, ""));
    alert("The puzzle's JSON was placed in your clipboard.")
})();
