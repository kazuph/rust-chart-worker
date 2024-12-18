# API Usage Guide

## Endpoint
```
POST https://your-worker.workers.dev/
```

## Request Specification

### Headers
```
Content-Type: application/json
```

### Request Body
```json
{
    "graph_type": "bar|scatter|line",
    "data": [1.0, 2.0, 3.0, 4.0, 5.0],
    "title": "Sample Chart",
    "x_label": "X-Axis",
    "y_label": "Y-Axis"
}
```

### Parameter Description

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| graph_type | string | ✓ | Chart type ("bar", "scatter", "line") |
| data | array[number] | ✓ | Numeric data array for chart plotting |
| title | string | - | Chart title |
| x_label | string | - | X-axis label |
| y_label | string | - | Y-axis label |

## Usage Examples

### Using cURL
```bash
curl -X POST https://your-worker.workers.dev/ \
  -H "Content-Type: application/json" \
  -d '{
    "graph_type": "bar",
    "data": [1.0, 2.0, 3.0, 4.0, 5.0],
    "title": "Sample Chart",
    "x_label": "X-Axis",
    "y_label": "Y-Axis"
  }' \
  --output chart.png
```

### Using JavaScript
```javascript
async function generateChart() {
  const response = await fetch('https://your-worker.workers.dev/', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      graph_type: 'line',
      data: [1.0, 2.0, 3.0, 4.0, 5.0],
      title: 'Sample Chart',
      x_label: 'X-Axis',
      y_label: 'Y-Axis'
    }),
  });

  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }

  const blob = await response.blob();
  const imageUrl = URL.createObjectURL(blob);

  // Example of displaying the image
  const img = document.createElement('img');
  img.src = imageUrl;
  document.body.appendChild(img);
}
```

### Using Python
```python
import requests
import matplotlib.pyplot as plt
import io

def generate_chart():
    response = requests.post(
        'https://your-worker.workers.dev/',
        json={
            'graph_type': 'scatter',
            'data': [1.0, 2.0, 3.0, 4.0, 5.0],
            'title': 'Sample Chart',
            'x_label': 'X-Axis',
            'y_label': 'Y-Axis'
        }
    )

    response.raise_for_status()

    # Example of displaying the image
    image_data = io.BytesIO(response.content)
    plt.imshow(plt.imread(image_data))
    plt.axis('off')
    plt.show()
```

## Error Responses

### Error Types

| HTTP Status | Description |
|-------------|-------------|
| 400 | Bad Request (Invalid JSON or missing required parameters) |
| 405 | Method Not Allowed |
| 500 | Internal Server Error |

### Error Response Example
```json
{
    "error": "Invalid JSON",
    "status": 400
}
```

## Limitations

1. Maximum data array size: 1000 elements
2. Maximum request body size: 100KB
3. Response timeout: 30 seconds
4. Rate limit: 100 requests per minute

## Best Practices

1. Error Handling
   - Implement error checking for all requests
   - Implement timeout handling

2. Performance
   - Use appropriate data sizes
   - Utilize caching

3. Security
   - Use HTTPS communication
   - Secure API key management (if required)
