#import <Foundation/Foundation.h>
#import <CoreSpotlight/CoreSpotlight.h>

NS_ASSUME_NONNULL_BEGIN

typedef void (^CSXCompletionHandler)(NSError * _Nullable error);

BOOL CSXTryBeginIndexBatch(CSSearchableIndex *index, NSError * _Nullable * _Nullable error);

BOOL CSXTryEndIndexBatch(
    CSSearchableIndex *index,
    NSData *clientState,
    CSXCompletionHandler completionHandler,
    NSError * _Nullable * _Nullable error
);

BOOL CSXTryEndIndexBatchWithExpectedClientState(
    CSSearchableIndex *index,
    NSData * _Nullable expectedClientState,
    NSData *newClientState,
    CSXCompletionHandler completionHandler,
    NSError * _Nullable * _Nullable error
) API_AVAILABLE(macos(15.0));

BOOL CSXTryStartQuery(CSSearchQuery *query, NSError * _Nullable * _Nullable error);

NS_ASSUME_NONNULL_END
