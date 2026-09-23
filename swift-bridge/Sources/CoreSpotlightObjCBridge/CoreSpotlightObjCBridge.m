#import "CoreSpotlightObjCBridge.h"

static void CSXStoreException(NSException *exception, NSError * _Nullable * _Nullable error) {
    if (error == NULL) {
        return;
    }
    NSString *name = exception.name ?: @"NSException";
    NSString *message = exception.reason.length > 0
        ? [NSString stringWithFormat:@"%@: %@", name, exception.reason]
        : name;
    *error = [NSError errorWithDomain:@"CoreSpotlightBridge"
                                 code:-1
                             userInfo:@{NSLocalizedDescriptionKey: message}];
}

BOOL CSXTryBeginIndexBatch(CSSearchableIndex *index, NSError * _Nullable * _Nullable error) {
    @try {
        [index beginIndexBatch];
        return YES;
    } @catch (NSException *exception) {
        CSXStoreException(exception, error);
        return NO;
    }
}

BOOL CSXTryEndIndexBatch(
    CSSearchableIndex *index,
    NSData *clientState,
    CSXCompletionHandler completionHandler,
    NSError * _Nullable * _Nullable error
) {
    @try {
        [index endIndexBatchWithClientState:clientState completionHandler:completionHandler];
        return YES;
    } @catch (NSException *exception) {
        CSXStoreException(exception, error);
        return NO;
    }
}

BOOL CSXTryEndIndexBatchWithExpectedClientState(
    CSSearchableIndex *index,
    NSData * _Nullable expectedClientState,
    NSData *newClientState,
    CSXCompletionHandler completionHandler,
    NSError * _Nullable * _Nullable error
) {
    @try {
        [index endIndexBatchWithExpectedClientState:expectedClientState
                                     newClientState:newClientState
                                  completionHandler:completionHandler];
        return YES;
    } @catch (NSException *exception) {
        CSXStoreException(exception, error);
        return NO;
    }
}

BOOL CSXTryStartQuery(CSSearchQuery *query, NSError * _Nullable * _Nullable error) {
    @try {
        [query start];
        return YES;
    } @catch (NSException *exception) {
        CSXStoreException(exception, error);
        return NO;
    }
}
