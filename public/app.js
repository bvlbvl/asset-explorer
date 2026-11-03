const form = document.querySelector("#search-form");
const input = document.querySelector("#isin-input");
const button = document.querySelector("#submit-button");
const statusMessage = document.querySelector("#status-message");
const chartWrap = document.querySelector("#chart-wrap");
const emptyState = document.querySelector("#empty-state");
const chartCanvas = document.querySelector("#history-chart");
let historyChart;

function setStatus(message, kind = "") {
  statusMessage.textContent = message;
  statusMessage.dataset.kind = kind;
}

function formatPrice(value, currency) {
  const formatted = new Intl.NumberFormat("de-DE", {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(value);
  return currency ? `${formatted} ${currency}` : formatted;
}

function renderChart(history) {
  if (historyChart) historyChart.destroy();

  const labels = history.points.map((point) => point.date);
  const values = history.points.map((point) => point.close);
  const context = chartCanvas.getContext("2d");
  const fill = context.createLinearGradient(0, 0, 0, 280);
  fill.addColorStop(0, "rgba(47, 118, 87, 0.20)");
  fill.addColorStop(1, "rgba(47, 118, 87, 0.00)");

  historyChart = new Chart(context, {
    type: "line",
    data: {
      labels,
      datasets: [{
        data: values,
        borderColor: "#2f7657",
        backgroundColor: fill,
        borderWidth: 2,
        pointRadius: 0,
        pointHoverRadius: 4,
        pointHoverBackgroundColor: "#d1ed70",
        fill: true,
        tension: 0.28,
      }],
    },
    options: {
      maintainAspectRatio: false,
      interaction: { intersect: false, mode: "index" },
      plugins: {
        legend: { display: false },
        tooltip: {
          displayColors: false,
          backgroundColor: "#1d2924",
          titleFont: { family: "IBM Plex Mono" },
          bodyFont: { family: "DM Sans" },
          callbacks: {
            label: (context) => formatPrice(context.parsed.y, history.currency),
          },
        },
      },
      scales: {
        x: {
          grid: { display: false },
          border: { display: false },
          ticks: {
            color: "#8a948d",
            maxTicksLimit: 7,
            maxRotation: 0,
            callback(value) {
              const date = new Date(`${this.getLabelForValue(value)}T00:00:00`);
              return new Intl.DateTimeFormat("de-DE", { day: "2-digit", month: "short" }).format(date);
            },
          },
        },
        y: {
          grid: { color: "rgba(29, 41, 36, 0.08)" },
          border: { display: false },
          ticks: {
            color: "#8a948d",
            maxTicksLimit: 5,
            callback: (value) => formatPrice(value, history.currency),
          },
        },
      },
    },
  });
}

form.addEventListener("submit", async (event) => {
  event.preventDefault();
  const isin = input.value.trim().toUpperCase();
  input.value = isin;
  button.disabled = true;
  setStatus("Schlusskurse werden geladen …");

  try {
    const response = await fetch(`/api/history?isin=${encodeURIComponent(isin)}`);
    const payload = await response.json();
    if (!response.ok) throw new Error(payload.error?.message ?? "Die Anfrage ist fehlgeschlagen.");

    document.querySelector("#instrument-name").textContent = payload.name;
    const currencyLabel = payload.currency ?? "Währung nicht angegeben";
    document.querySelector("#instrument-meta").textContent = `${payload.isin}  ·  ${payload.symbol}  ·  ${payload.exchange}  ·  ${currencyLabel}`;
    document.querySelector("#range-label").innerHTML = `${payload.from} <span>–</span> ${payload.to} · TAGESWERTE`;
    document.querySelector("#latest-close").textContent = formatPrice(payload.points.at(-1).close, payload.currency);
    document.querySelector("#point-count").textContent = String(payload.points.length).padStart(2, "0");
    emptyState.hidden = true;
    chartWrap.hidden = false;
    renderChart(payload);
    setStatus("Tägliche Schlusskurse · Quelle: Marketstack", "success");
  } catch (error) {
    chartWrap.hidden = true;
    emptyState.hidden = false;
    emptyState.querySelector("p").textContent = error.message;
    document.querySelector("#instrument-name").textContent = "Keine Kursdaten geladen";
    document.querySelector("#instrument-meta").textContent = isin;
    document.querySelector("#latest-close").textContent = "--";
    document.querySelector("#point-count").textContent = "--";
    if (historyChart) {
      historyChart.destroy();
      historyChart = undefined;
    }
    setStatus(error.message, "error");
  } finally {
    button.disabled = false;
  }
});