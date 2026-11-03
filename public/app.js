const form = document.querySelector("#search-form");
const input = document.querySelector("#isin-input");
const button = document.querySelector("#submit-button");
const statusMessage = document.querySelector("#status-message");
const chartWrap = document.querySelector("#chart-wrap");
const emptyState = document.querySelector("#empty-state");
const chartCanvas = document.querySelector("#history-chart");
const chartHud = document.querySelector("#chart-hud");
const debugOutput = document.querySelector("#debug-output");
const debugSteps = document.querySelector("#debug-steps");
let historyChart;
let activeHistory = null;
let selectedChartIndex = null;

function clamp(value, min, max) {
  return Math.min(Math.max(value, min), max);
}

function setStatus(message, kind = "") {
  statusMessage.textContent = message;
  statusMessage.dataset.kind = kind;
}

function renderDebugOutput(steps) {
  debugSteps.replaceChildren();
  if (!Array.isArray(steps) || steps.length === 0) {
    debugOutput.hidden = true;
    return;
  }

  for (const step of steps) {
    const item = document.createElement("li");
    item.textContent = step;
    debugSteps.append(item);
  }
  debugOutput.hidden = false;
}

function formatPrice(value, currency) {
  const formatted = new Intl.NumberFormat("de-DE", {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(value);
  return currency ? `${formatted} ${currency}` : formatted;
}

function formatPercent(value) {
  const sign = value >= 0 ? "+" : "";
  return `${sign}${value.toFixed(1)}%`;
}

function calculateGrowthPercent(startValue, currentValue) {
  if (!Number.isFinite(startValue) || !Number.isFinite(currentValue) || startValue === 0) {
    return 0;
  }

  return ((currentValue - startValue) / startValue) * 100;
}

function showChartHud(history, chart, index) {
  if (!history || !history.points || !history.points[index]) {
    chartHud.hidden = true;
    return;
  }

  const point = history.points[index];
  const startValue = history.points[0].close;
  const growthPercent = calculateGrowthPercent(startValue, point.close);
  const xPosition = chart.scales.x.getPixelForValue(index);
  const left = clamp(xPosition + chart.canvas.offsetLeft, 92, chartWrap.clientWidth - 92);
  const dateText = new Intl.DateTimeFormat("de-DE", {
    day: "2-digit",
    month: "short",
    year: "numeric",
  }).format(new Date(`${point.date}T00:00:00`));

  chartHud.hidden = false;
  chartHud.style.left = `${left}px`;
  chartHud.style.top = "18px";
  chartHud.innerHTML = `
    <div class="chart-hud-header">${dateText}</div>
    <div class="chart-hud-meta">
      <span>${formatPrice(point.close, history.currency)}</span>
      <span class="${growthPercent >= 0 ? "chart-hud-positive" : "chart-hud-negative"}">${formatPercent(growthPercent)}</span>
    </div>
  `;
}

function getNearestIndexFromPointer(event, chart) {
  const canvasRect = chart.canvas.getBoundingClientRect();
  const pointerX = (event?.native?.clientX ?? event?.clientX ?? canvasRect.left) - canvasRect.left;
  let nearestIndex = 0;
  let nearestDistance = Number.POSITIVE_INFINITY;

  for (let index = 0; index < chart.data.labels.length; index += 1) {
    const x = chart.scales.x.getPixelForValue(index);
    const distance = Math.abs(pointerX - x);
    if (distance < nearestDistance) {
      nearestDistance = distance;
      nearestIndex = index;
    }
  }

  return nearestIndex;
}

function updateSelectedPoint(event, chart, history) {
  const active = chart.getElementsAtEventForMode(event, "nearest", { intersect: false, axis: "x" }, true);
  const index = active.length ? active[0].index : getNearestIndexFromPointer(event, chart);

  if (!Number.isInteger(index) || !history?.points?.[index]) {
    if (selectedChartIndex === null || !history?.points?.[selectedChartIndex]) {
      chartHud.hidden = true;
      return;
    }
    showChartHud(history, chart, selectedChartIndex);
    return;
  }

  selectedChartIndex = index;
  showChartHud(history, chart, selectedChartIndex);
}

function renderChart(history) {
  if (historyChart) historyChart.destroy();
  activeHistory = history;
  selectedChartIndex = null;
  chartHud.hidden = true;

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
        pointHitRadius: 12,
        pointHoverBackgroundColor: "#d1ed70",
        fill: true,
        tension: 0.28,
      }],
    },
    options: {
      maintainAspectRatio: false,
      interaction: { intersect: false, mode: "nearest", axis: "x" },
      onHover: (event, activeElements, chart) => {
        if (!event) return;
        updateSelectedPoint(event, chart, history);
      },
      onClick: (event, activeElements, chart) => {
        if (!event) return;
        const active = chart.getElementsAtEventForMode(event, "nearest", { intersect: false, axis: "x" }, true);
        const fallbackIndex = active.length ? active[0].index : getNearestIndexFromPointer(event, chart);
        selectedChartIndex = fallbackIndex;
        showChartHud(history, chart, selectedChartIndex);
      },
      plugins: {
        legend: { display: false },
        tooltip: {
          enabled: false,
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
  renderDebugOutput(null);
  setStatus("Schlusskurse werden geladen …");

  try {
    const response = await fetch(`/api/history?isin=${encodeURIComponent(isin)}`);
    const payload = await response.json();
    if (!response.ok) {
      renderDebugOutput(payload.error?.debug);
      throw new Error(payload.error?.message ?? "Die Anfrage ist fehlgeschlagen.");
    }

    renderDebugOutput(payload.debug);
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