Array.from(document.querySelectorAll('td:nth-child(3)')).forEach(cell => {
    const dates = JSON.parse(cell.innerText);

    const prettyStartDate = !!dates[0] ?  (() => {
        const d = new Date((parseInt(dates[0])+new Date().getTimezoneOffset()*60)*1000);
        const month = ["January","February","March","April","May","June","July","August","Sepember","October","November","December"][d.getMonth()];
        return `${d.getDate()} ${month} ${d.getFullYear()}`;
    })() : "<i>TBD</i>";
    const prettyEndDate = !dates[0] ? "" : (!!dates[1] ? (() => {
        const d = new Date((parseInt(dates[1])+new Date().getTimezoneOffset()*60)*1000);
        const month = ["January","February","March","April","May","June","July","August","Sepember","October","November","December"][d.getMonth()];
        return ` – ${d.getDate()} ${month} ${d.getFullYear()}`;
    })() : " – <i>ongoing</i>");

    cell.innerHTML = `${prettyStartDate}${prettyEndDate}`;
});