/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import XCTest

@testable import Glean

final class PingUploadSchedulerTests: XCTestCase {
    let testPingUploadRequest = PingRequest(
        documentId: "Some ID",
        path: "Some path",
        body: [],
        headers: [:],
        bodyHasInfoSections: true,
        pingName: "Ping name",
        uploaderCapabilities: []
    )

    func testPingUploadScheduler_doesNotInfiniteLoop_onProcess() {
        let subject = createSubject()

        subject.process()

        let waitForSerialOperationQueueExpectation = expectation(description: "Wait for all operations to finish")

        // Ensure the processing done on the serial operation queue does not loop indefinitely. We wait on a background
        // thread so the test can timeout if the loop never returns.
        DispatchQueue.global().async {
            Dispatchers.shared.serialOperationQueue.waitUntilAllOperationsAreFinished()
            waitForSerialOperationQueueExpectation.fulfill()
        }

        wait(for: [waitForSerialOperationQueueExpectation], timeout: 2)
    }

    func testPingUploadScheduler_endsBackgroundTasks_whenFinished() {
        let mockBackgroundTaskScheduler = MockBackgroundTaskScheduler(withValidTaskIdentifier: true)

        let subject = createSubject(backgroundTaskScheduler: mockBackgroundTaskScheduler)

        subject.process()

        let waitForSerialOperationQueueExpectation = expectation(description: "Wait for all operations to finish")

        DispatchQueue.global().async {
            Dispatchers.shared.serialOperationQueue.waitUntilAllOperationsAreFinished()

            // We expect that a successful `process()` call will start and end a background task exactly once
            XCTAssertEqual(mockBackgroundTaskScheduler.calledBeginBackgroundTask, 1)
            XCTAssertEqual(mockBackgroundTaskScheduler.calledEndBackgroundTask, 1)

            waitForSerialOperationQueueExpectation.fulfill()
        }

        wait(for: [waitForSerialOperationQueueExpectation], timeout: 2)
    }

    func testPingUploadScheduler_doesNotEndBackgroundTasks_forInvalidTaskIdentifier() {
        let mockBackgroundTaskScheduler = MockBackgroundTaskScheduler(withValidTaskIdentifier: false)

        let subject = createSubject(backgroundTaskScheduler: mockBackgroundTaskScheduler)

        subject.process()

        let waitForSerialOperationQueueExpectation = expectation(description: "Wait for all operations to finish")

        DispatchQueue.global().async {
            Dispatchers.shared.serialOperationQueue.waitUntilAllOperationsAreFinished()

            // If the background scheduler returns a `.invalid` identifier, we should never try to end the background
            // task.
            XCTAssertEqual(mockBackgroundTaskScheduler.calledBeginBackgroundTask, 1)
            XCTAssertEqual(mockBackgroundTaskScheduler.calledEndBackgroundTask, 0)

            waitForSerialOperationQueueExpectation.fulfill()
        }

        wait(for: [waitForSerialOperationQueueExpectation], timeout: 2)
    }

    func testPingUploadScheduler_forUploadTasks_callsPingUploader() {
        let testTaskType = PingUploadTask.upload(request: testPingUploadRequest)

        let pingUploadExpectation = expectation(description: "Wait for the ping upload request")

        let mockBackgroundTaskScheduler = MockBackgroundTaskScheduler(withValidTaskIdentifier: false)
        let mockGleanUploadTaskProvider = MockGleanUploadTaskProviderProtocol(returningTask: testTaskType)
        let mockPingUploader = MockPingUploader(
            uploadRequested: { _ in
                // We want to ensure that we try to upload a ping for `PingUploadTask.upload` tasks
                pingUploadExpectation.fulfill()
            }
        )

        let subject = createSubject(
            mockPingUploader: mockPingUploader,
            backgroundTaskScheduler: mockBackgroundTaskScheduler,
            gleanUploadTaskProvider: mockGleanUploadTaskProvider
        )

        subject.process()

        wait(for: [pingUploadExpectation], timeout: 2)
    }

    func testPingUploadScheduler_forWaitTasks() {
        let testTaskType = PingUploadTask.wait(time: 1)

        let mockBackgroundTaskScheduler = MockBackgroundTaskScheduler(withValidTaskIdentifier: true)
        let mockGleanUploadTaskProvider = MockGleanUploadTaskProviderProtocol(returningTask: testTaskType)

        let subject = createSubject(
            backgroundTaskScheduler: mockBackgroundTaskScheduler,
            gleanUploadTaskProvider: mockGleanUploadTaskProvider
        )

        subject.process()

        let waitForSerialOperationQueueExpectation = expectation(description: "Wait for all operations to finish")

        DispatchQueue.global().async {
            Dispatchers.shared.serialOperationQueue.waitUntilAllOperationsAreFinished()

            // Our mock provides a `PingUploadTask.done` after the `PingUploadTask.wait`, so expect background tasks
            // to end.
            XCTAssertEqual(mockBackgroundTaskScheduler.calledBeginBackgroundTask, 1)
            XCTAssertEqual(mockBackgroundTaskScheduler.calledEndBackgroundTask, 1)

            waitForSerialOperationQueueExpectation.fulfill()
        }

        wait(for: [waitForSerialOperationQueueExpectation], timeout: 2)
    }

    func testPingUploadScheduler_forDoneTasks() {
        let testTaskType = PingUploadTask.done(unused: 0)

        let mockBackgroundTaskScheduler = MockBackgroundTaskScheduler(withValidTaskIdentifier: true)
        let mockGleanUploadTaskProvider = MockGleanUploadTaskProviderProtocol(returningTask: testTaskType)

        let subject = createSubject(
            backgroundTaskScheduler: mockBackgroundTaskScheduler,
            gleanUploadTaskProvider: mockGleanUploadTaskProvider
        )

        subject.process()

        let waitForSerialOperationQueueExpectation = expectation(description: "Wait for all operations to finish")

        DispatchQueue.global().async {
            Dispatchers.shared.serialOperationQueue.waitUntilAllOperationsAreFinished()

            // Our mock provides a `PingUploadTask.done` after the `PingUploadTask.wait`, so expect background tasks
            // to end.
            XCTAssertEqual(mockBackgroundTaskScheduler.calledBeginBackgroundTask, 1)
            XCTAssertEqual(mockBackgroundTaskScheduler.calledEndBackgroundTask, 1)

            waitForSerialOperationQueueExpectation.fulfill()
        }

        wait(for: [waitForSerialOperationQueueExpectation], timeout: 2)
    }

    // MARK: Helpers

    func createSubject(
        mockPingUploader: MockPingUploader? = nil,
        backgroundTaskScheduler: MockBackgroundTaskScheduler? = nil,
        gleanUploadTaskProvider: MockGleanUploadTaskProviderProtocol? = nil
    ) -> PingUploadScheduler {
        let configuration = Configuration(
            httpClient: mockPingUploader
                        ?? MockPingUploader(uploadRequested: { _ in })
        )

        let subject = PingUploadScheduler(
            configuration: configuration,
            backgroundTaskScheduler: backgroundTaskScheduler
                                     ?? MockBackgroundTaskScheduler(withValidTaskIdentifier: true),
            gleanUploadTaskProvider: gleanUploadTaskProvider
                                     ?? MockGleanUploadTaskProviderProtocol(returningTask: .done(unused: 0))
        )

        return subject
    }
}
