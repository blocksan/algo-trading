// import-csv-to-mongodb.js

const fs = require('fs');
const csv = require('csv-parser');
const { MongoClient } = require('mongodb');

// Configuration
const csvFilePath = './ACC_with_indicators_.csv'; // Update with your CSV file path
const mongoUrl = 'mongodb://localhost:27017';
const dbName = 'algo_trading';
const collectionName = 'daily_stock_data';

// Connect to MongoDB
async function importCsvToMongo() {
  const client = new MongoClient(mongoUrl);
  
  try {
    await client.connect();
    console.log('Connected to MongoDB');
    
    const db = client.db(dbName);
    const collection = db.collection(collectionName);
    
    // Create an array to store the parsed data
    const stockData = [];
    
    // Read and parse the CSV file
    await new Promise((resolve, reject) => {
      fs.createReadStream(csvFilePath)
        .pipe(csv())
        .on('data', (row) => {
          // Extract only the fields we want
          const stockEntry = {
            symbol: 'ACC',
            date: row.date,
            open: parseFloat(row.open),
            high: parseFloat(row.high),
            low: parseFloat(row.low),
            close: parseFloat(row.close),
            volume: parseInt(row.volume || '0', 10),
            market_time_frame: '5Min'
          };
          
          stockData.push(stockEntry);
        })
        .on('end', () => {
          console.log(`CSV file successfully processed. Found ${stockData.length} records.`);
          resolve();
        })
        .on('error', (error) => {
          reject(error);
        });
    });
    
    // Insert the data into MongoDB
    if (stockData.length > 0) {
      const result = await collection.insertMany(stockData);
      console.log(`${result.insertedCount} documents were inserted into MongoDB.`);
    } else {
      console.log('No data to insert.');
    }
    
  } catch (error) {
    console.error('Error:', error);
  } finally {
    await client.close();
    console.log('MongoDB connection closed.');
  }
}

// Run the import function
importCsvToMongo().catch(console.error);