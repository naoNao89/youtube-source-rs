const WebSocket = require('ws');
const axios = require('axios');
const express = require('express');
const fs = require('fs').promises;
const path = require('path');

class LavalinkTestBot {
    constructor() {
        this.app = express();
        this.testResults = [];
        this.connections = new Map();
        
        // Configuration
        this.config = {
            v4: {
                host: process.env.LAVALINK_V4_HOST || 'localhost',
                port: process.env.LAVALINK_V4_PORT || 2333,
                password: process.env.LAVALINK_PASSWORD || 'youshallnotpass'
            },
            v3: {
                host: process.env.LAVALINK_V3_HOST || 'localhost',
                port: process.env.LAVALINK_V3_PORT || 2334,
                password: process.env.LAVALINK_PASSWORD || 'youshallnotpass'
            }
        };
        
        this.setupHealthCheck();
    }
    
    setupHealthCheck() {
        this.app.get('/health', (req, res) => {
            res.json({ status: 'ok', timestamp: new Date().toISOString() });
        });
        
        this.app.listen(3001, () => {
            console.log('Test bot health check server running on port 3001');
        });
    }
    
    async connectToLavalink(version) {
        const config = this.config[version];
        const wsUrl = `ws://${config.host}:${config.port}/v4/websocket`;
        
        return new Promise((resolve, reject) => {
            const ws = new WebSocket(wsUrl, {
                headers: {
                    'Authorization': config.password,
                    'User-Id': '123456789',
                    'Client-Name': `youtube-rust-test-bot-${version}`
                }
            });
            
            ws.on('open', () => {
                console.log(`Connected to Lavalink ${version.toUpperCase()}`);
                this.connections.set(version, ws);
                resolve(ws);
            });
            
            ws.on('error', (error) => {
                console.error(`Failed to connect to Lavalink ${version.toUpperCase()}:`, error);
                reject(error);
            });
            
            ws.on('message', (data) => {
                try {
                    const message = JSON.parse(data);
                    this.handleMessage(version, message);
                } catch (error) {
                    console.error(`Error parsing message from ${version}:`, error);
                }
            });
        });
    }
    
    handleMessage(version, message) {
        console.log(`[${version.toUpperCase()}] Received:`, JSON.stringify(message, null, 2));
        
        // Store relevant messages for test analysis
        if (message.op === 'ready' || message.op === 'stats' || message.op === 'event') {
            this.testResults.push({
                version,
                timestamp: new Date().toISOString(),
                type: message.op,
                data: message
            });
        }
    }
    
    async testLoadTrack(version, identifier) {
        const config = this.config[version];
        const url = `http://${config.host}:${config.port}/v4/loadtracks`;
        
        try {
            const response = await axios.get(url, {
                params: { identifier },
                headers: {
                    'Authorization': config.password
                }
            });
            
            const result = {
                version,
                test: 'loadTrack',
                identifier,
                timestamp: new Date().toISOString(),
                success: response.status === 200,
                data: response.data
            };
            
            this.testResults.push(result);
            console.log(`[${version.toUpperCase()}] Load track test:`, result.success ? 'PASS' : 'FAIL');
            
            return result;
        } catch (error) {
            const result = {
                version,
                test: 'loadTrack',
                identifier,
                timestamp: new Date().toISOString(),
                success: false,
                error: error.message
            };
            
            this.testResults.push(result);
            console.error(`[${version.toUpperCase()}] Load track test failed:`, error.message);
            
            return result;
        }
    }
    
    async testSearch(version, query) {
        const config = this.config[version];
        const url = `http://${config.host}:${config.port}/v4/loadtracks`;
        
        try {
            const response = await axios.get(url, {
                params: { identifier: `ytsearch:${query}` },
                headers: {
                    'Authorization': config.password
                }
            });
            
            const result = {
                version,
                test: 'search',
                query,
                timestamp: new Date().toISOString(),
                success: response.status === 200 && response.data.tracks && response.data.tracks.length > 0,
                data: response.data
            };
            
            this.testResults.push(result);
            console.log(`[${version.toUpperCase()}] Search test:`, result.success ? 'PASS' : 'FAIL');
            
            return result;
        } catch (error) {
            const result = {
                version,
                test: 'search',
                query,
                timestamp: new Date().toISOString(),
                success: false,
                error: error.message
            };
            
            this.testResults.push(result);
            console.error(`[${version.toUpperCase()}] Search test failed:`, error.message);
            
            return result;
        }
    }
    
    async testPlaylist(version, playlistId) {
        const config = this.config[version];
        const url = `http://${config.host}:${config.port}/v4/loadtracks`;
        
        try {
            const response = await axios.get(url, {
                params: { identifier: `https://www.youtube.com/playlist?list=${playlistId}` },
                headers: {
                    'Authorization': config.password
                }
            });
            
            const result = {
                version,
                test: 'playlist',
                playlistId,
                timestamp: new Date().toISOString(),
                success: response.status === 200 && response.data.tracks && response.data.tracks.length > 0,
                data: response.data
            };
            
            this.testResults.push(result);
            console.log(`[${version.toUpperCase()}] Playlist test:`, result.success ? 'PASS' : 'FAIL');
            
            return result;
        } catch (error) {
            const result = {
                version,
                test: 'playlist',
                playlistId,
                timestamp: new Date().toISOString(),
                success: false,
                error: error.message
            };
            
            this.testResults.push(result);
            console.error(`[${version.toUpperCase()}] Playlist test failed:`, error.message);
            
            return result;
        }
    }
    
    async runAllTests() {
        console.log('Starting comprehensive Lavalink YouTube Rust plugin tests...');
        
        const testCases = [
            // Video loading tests
            { type: 'video', id: 'dQw4w9WgXcQ' },
            { type: 'video', id: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ' },
            { type: 'video', id: 'https://youtu.be/dQw4w9WgXcQ' },
            
            // Search tests
            { type: 'search', query: 'never gonna give you up' },
            { type: 'search', query: 'rick astley' },
            
            // Playlist tests (using a known public playlist)
            { type: 'playlist', id: 'PLFgquLnL59alCl_2TQvOiD5Vgm1hCaGSI' }
        ];
        
        for (const version of ['v4', 'v3']) {
            console.log(`\n=== Testing Lavalink ${version.toUpperCase()} ===`);
            
            try {
                await this.connectToLavalink(version);
                
                // Wait for connection to stabilize
                await new Promise(resolve => setTimeout(resolve, 2000));
                
                for (const testCase of testCases) {
                    switch (testCase.type) {
                        case 'video':
                            await this.testLoadTrack(version, testCase.id);
                            break;
                        case 'search':
                            await this.testSearch(version, testCase.query);
                            break;
                        case 'playlist':
                            await this.testPlaylist(version, testCase.id);
                            break;
                    }
                    
                    // Small delay between tests
                    await new Promise(resolve => setTimeout(resolve, 1000));
                }
                
            } catch (error) {
                console.error(`Failed to test ${version}:`, error);
            }
        }
        
        await this.saveResults();
        this.printSummary();
    }
    
    async saveResults() {
        const resultsPath = path.join('/app/test-results', 'integration-test-results.json');
        
        try {
            await fs.writeFile(resultsPath, JSON.stringify({
                timestamp: new Date().toISOString(),
                results: this.testResults,
                summary: this.generateSummary()
            }, null, 2));
            
            console.log(`\nTest results saved to: ${resultsPath}`);
        } catch (error) {
            console.error('Failed to save test results:', error);
        }
    }
    
    generateSummary() {
        const summary = {
            total: this.testResults.length,
            passed: this.testResults.filter(r => r.success).length,
            failed: this.testResults.filter(r => !r.success).length,
            byVersion: {},
            byTest: {}
        };
        
        // Group by version
        for (const result of this.testResults) {
            if (!summary.byVersion[result.version]) {
                summary.byVersion[result.version] = { total: 0, passed: 0, failed: 0 };
            }
            summary.byVersion[result.version].total++;
            if (result.success) {
                summary.byVersion[result.version].passed++;
            } else {
                summary.byVersion[result.version].failed++;
            }
        }
        
        // Group by test type
        for (const result of this.testResults) {
            const testType = result.test || result.type;
            if (!summary.byTest[testType]) {
                summary.byTest[testType] = { total: 0, passed: 0, failed: 0 };
            }
            summary.byTest[testType].total++;
            if (result.success) {
                summary.byTest[testType].passed++;
            } else {
                summary.byTest[testType].failed++;
            }
        }
        
        return summary;
    }
    
    printSummary() {
        const summary = this.generateSummary();
        
        console.log('\n=== TEST SUMMARY ===');
        console.log(`Total Tests: ${summary.total}`);
        console.log(`Passed: ${summary.passed}`);
        console.log(`Failed: ${summary.failed}`);
        console.log(`Success Rate: ${((summary.passed / summary.total) * 100).toFixed(2)}%`);
        
        console.log('\nBy Version:');
        for (const [version, stats] of Object.entries(summary.byVersion)) {
            console.log(`  ${version.toUpperCase()}: ${stats.passed}/${stats.total} passed`);
        }
        
        console.log('\nBy Test Type:');
        for (const [testType, stats] of Object.entries(summary.byTest)) {
            console.log(`  ${testType}: ${stats.passed}/${stats.total} passed`);
        }
    }
}

// Start the test bot
const bot = new LavalinkTestBot();

// Wait for services to be ready, then run tests
setTimeout(async () => {
    try {
        await bot.runAllTests();
        console.log('\nAll tests completed. Bot will continue running for monitoring...');
    } catch (error) {
        console.error('Test execution failed:', error);
        process.exit(1);
    }
}, 10000); // Wait 10 seconds for services to start
